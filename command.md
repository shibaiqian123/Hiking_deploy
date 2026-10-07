# port
ip -br link

# connect：
sudo nmcli con add type ethernet ifname enx6c1ff7cf9f63 con-name g1-robot \
    ipv4.method manual ipv4.addresses 192.168.123.222/24.0


# deploy
 ./bootstrap_env.sh

# sim 
source sim2sim/env_sim.sh
mujoco:  python sim2sim/g1_mujoco_bridge.py 
deploy:  python instinct_onboard/scripts/g1_parkour_laptop.py --zmq_addr tcp://127.0.0.1:5556 --nodryrun

multi-scene (full training terrain, noise & walls disabled by default):
# regenerate only if needed — the .mjb ships with the repo
.venv-terrain/bin/python sim2sim/gen_training_terrain.py
python sim2sim/g1_mujoco_bridge.py \
    --scene sim2sim/assets/training_terrain_scene.mjb \
    --spawn_x -12.00 --spawn_y -36.00 --spawn_height 0.760
