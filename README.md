# Hiking Deploy — "Hiking in the Wild" on Unitree G1

Deployment bundle for running the **Hiking-in-the-Wild** perceptive parkour /
stair-climbing policies on a real Unitree G1 (29-DoF) humanoid — either fully
onboard (Jetson Orin NX) or **off-board from an external laptop** wired to the
robot, with depth streamed over the network and motor commands sent over DDS.

```
depth (RealSense D435i on robot) ──ZMQ──►┐
/lowstate, /secondary_imu, joystick ─DDS─►│  laptop: obs pipeline + ONNX policy
                                          │  (50 Hz)
robot motor-level PD ◄────── /lowcmd ─DDS─┘
```

## Repository Layout

| Path | What it is |
|---|---|
| [`instinct_onboard/`](instinct_onboard/) | The deployment framework (vendored from [project-instinct/instinct_onboard](https://github.com/project-instinct/instinct_onboard), with off-board additions). **Start with its [README](instinct_onboard/README.md)** — full architecture, installation and usage docs live there. |
| [`hiking-in-the-wild_Data&Model/`](hiking-in-the-wild_Data%26Model/) | Deployable checkpoints: **stand** policy and **stair-parkour** policy (`actor.onnx` + `0-depth_encoder.onnx` + the training `env.yaml` each), plus motion reference data. |
| [`setup_env.sh`](setup_env.sh) | One-shot environment setup for the off-board laptop: ROS2 Humble + unitree msgs + CycloneDDS (bound to the robot NIC) + Python venv + default checkpoint paths. Edit the NIC name / paths for your machine. |
| [`sim2sim/`](sim2sim/) | **MuJoCo robot impersonator** for validating the whole deployment stack before touching the real robot: publishes `/lowstate`/IMU, runs the motor PD, streams rendered depth over ZMQ, keyboard acts as the joystick — the deploy script runs unmodified against it (DDS bound to loopback for safety). Ships with the training-spec staircase scene and a generator for the **official training terrain** (all 10 sub-terrain types from InstinctMJ's parkour config, heightfield collisions). See [sim2sim/README.md](sim2sim/README.md). |

Not tracked (rebuild locally, see [instinct_onboard/README.md](instinct_onboard/README.md)
"Installation — Mode B"): `.venv/` (Python env), `unitree_ros2/` (colcon
workspace: unitree_hg/go msgs + CycloneDDS + rmw), `g1_crc/` (CRC module
sources; the compiled `crc_module.so` is architecture-specific),
`InstinctMJ/` (upstream checkout used only by the terrain generator) and
`sim2sim/assets/` (generated terrain scenes — rerun
`sim2sim/gen_training_terrain.py`).

## Additions on top of upstream instinct_onboard

- **`ros_nodes/zmq_camera.py`** — `ZMQCamera`: consumes the robot Jetson's raw
  z16 ZMQ depth broadcast (mm → metres, resolution adaptation), a drop-in
  replacement for the local RealSense camera class.
- **`scripts/g1_parkour_laptop.py`** — off-board entry script: identical state
  machine to the onboard `g1_parkour.py`, camera source swapped to `ZMQCamera`.
- **`Tools/real_time_img.py`** — side-by-side viewer of the RAW depth image vs
  the exact processed observation the policy sees (params parsed from the
  checkpoint's `env.yaml`); supports local RealSense and ZMQ sources.
- Rewritten [instinct_onboard/README.md](instinct_onboard/README.md) covering
  both deployment modes end to end.

## Quick Start

One-time setup: follow "Installation — Mode B" in
[instinct_onboard/README.md](instinct_onboard/README.md), and generate the
terrain once for sim2sim (see [sim2sim/README.md](sim2sim/README.md)).

### 1. Sim2sim (validate everything in MuJoCo first)

```bash
# terminal 1 — simulated robot (flat ground + training-spec staircase)
cd /home/galbot/Intern/Hiking_wild
source sim2sim/env_sim.sh                            # loopback DDS — can never reach a real robot
python sim2sim/g1_mujoco_bridge.py

# optional instead: the official training terrain (10 types x 4 difficulty rows;
# +/- switch difficulty, * / switch type — needs the one-time terrain generation)
# python sim2sim/g1_mujoco_bridge.py \
#     --scene sim2sim/assets/training_terrain_scene.mjb \
#     --spawn_x -12.00 --spawn_y -36.00 --spawn_height 0.760

# terminal 2 — the UNMODIFIED deploy script
cd /home/galbot/Intern/Hiking_wild
source sim2sim/env_sim.sh
python instinct_onboard/scripts/g1_parkour_laptop.py --zmq_addr tcp://127.0.0.1:5556 --nodryrun

# terminal 3 (optional) — live depth pipeline viewer
cd /home/galbot/Intern/Hiking_wild
source sim2sim/env_sim.sh
real_time_img --source zmq --zmq-addr tcp://127.0.0.1:5556 --stages
```

Drive with the viewer-window keyboard: `Enter` wake → wait for "ColdStartAgent
done" → `7` stand → `8` release gantry → `6` parkour → `↑` forward;
`+`/`-` difficulty rows, `*`//` terrain types, `.` camera follow/free,
`E` e-stop test. Full table in [sim2sim/README.md](sim2sim/README.md).

### 2. Real robot

```bash
# terminal 1 — deploy (dryrun FIRST: robot never moves, verify 50 Hz + sane q/kp/kd)
cd /home/galbot/Intern/Hiking_wild
source setup_env.sh                                  # binds DDS to the robot NIC
python instinct_onboard/scripts/g1_parkour_laptop.py
#   verify: ros2 topic hz /lowcmd_dryrun_XXXXX  → stable 50 Hz
# then power the motors:
python instinct_onboard/scripts/g1_parkour_laptop.py --nodryrun

# terminal 2 (optional) — live depth pipeline viewer
cd /home/galbot/Intern/Hiking_wild
source setup_env.sh
real_time_img --source zmq --stages
```

**Never mix the env scripts**: `setup_env.sh` = real robot, `sim2sim/env_sim.sh`
= simulation. Running `--nodryrun` with `setup_env.sh` sourced commands the
real motors.

### Wireless controller operations (real robot)

**Before launching the script** — put the robot into its firmware debug mode
(robot lying flat, one person holding it afterwards):

| Step | Buttons | Confirmation |
|---|---|---|
| 1. Enter debug mode (disables the built-in motion service — REQUIRED, otherwise `/lowcmd` never reaches the motors) | **L2+R2** together | Robot **voice-announces debug mode** and goes limp. No announcement? It may already be in debug mode — joints can be moved by hand when it is. Can also be checked/toggled in the Unitree app (motion service off). |
| 2. Joint reset | **L2+A**, then **L2+B** | Joints reset, damped state |
| 3. Stand the robot up | — | Person holds it upright, feet on the ground |

**After launching the script:**

| Step | Buttons | Wait for (terminal) |
|---|---|---|
| Wake the script | **any button** (e.g. A — not L2/R2) | `All necessary buffers received`; cold start begins ramping joints — keep holding |
| Wait for convergence | — | `ColdStartAgent done, press 'R1'` (the `max error` line is your progress bar; if it freezes, that joint is physically stuck — e-stop and free it) |
| Stand policy | **R1** (hold ~1 s) | `switching to stand agent`; let go gradually |
| Parkour policy | **L1** | `switching to parkour agent` |
| Walk | **left stick** forward = 0.5 m/s (past half travel; release = stop); **right stick** = yaw, proportional | — |
| Back to stand | **R1** | anytime from parkour |
| **EMERGENCY STOP** | **L2 or R2, single press, anytime** | Motors go limp instantly, script exits — the robot collapses, **have a person or gantry ready** |

Backward / lateral sticks are disabled by default (`--lin_vel_range` etc. to
change). If buttons seem dead, the controller may have auto-slept — nudge a
stick first.

## Safety

Dryrun is the default everywhere; `--nodryrun` is an explicit act. Read the
Safety Notes in [instinct_onboard/README.md](instinct_onboard/README.md) before
powering the motors.

## Acknowledgements

- Deployment framework: [project-instinct/instinct_onboard](https://github.com/project-instinct/instinct_onboard)
- CRC module: [ZiwenZhuang/g1_crc](https://github.com/ZiwenZhuang/g1_crc)
- Messages / DDS: [unitreerobotics/unitree_ros2](https://github.com/unitreerobotics/unitree_ros2)
