#!/usr/bin/env bash
# One-shot environment bootstrap for this deployment repo (Mode B, off-board computer).
# Automates: ROS2 Humble (apt) -> python venv -> unitree_ros2 colcon workspace -> crc_module.
# Re-runnable: each stage checks whether it is already done and skips itself.
#
#   ./bootstrap_env.sh          # run everything (apt stages will ask for sudo)
#
# Afterwards: edit the NIC name in setup_env.sh if needed, then `source setup_env.sh`.
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

stage() { echo; echo "==== $* ===="; }

# ---------------------------------------------------------------- 0. prerequisites
stage "0/6 prerequisites"
if ! grep -q 'VERSION_ID="22.04"' /etc/os-release; then
    echo "ERROR: this stack requires Ubuntu 22.04 (ROS2 Humble). Current system:"
    grep PRETTY_NAME /etc/os-release
    exit 1
fi
if ! command -v uv > /dev/null; then
    echo "uv not found — installing to ~/.local/bin ..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    export PATH="$HOME/.local/bin:$PATH"
fi
command -v git > /dev/null || { echo "installing git ..."; sudo apt update && sudo apt install -y git; }
# conda pythons shadow /usr/bin/python3 and break ament/colcon builds — strip them from PATH
if [[ "$(command -v python3)" == *conda* ]]; then
    echo "conda python detected in PATH — removing conda entries for this run"
    PATH="$(echo "$PATH" | tr ':' '\n' | grep -v conda | paste -sd':')"
    export PATH
fi
echo "prerequisites OK (python3 = $(command -v python3))"

# ---------------------------------------------------------------- 1. ROS2 Humble
stage "1/6 ROS2 Humble"
if [ -d /opt/ros/humble ]; then
    echo "already installed — skip"
else
    sudo apt update
    sudo apt install -y software-properties-common curl
    sudo add-apt-repository -y universe
    sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
        -o /usr/share/keyrings/ros-archive-keyring.gpg
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo "$UBUNTU_CODENAME") main" \
        | sudo tee /etc/apt/sources.list.d/ros2.list > /dev/null
    sudo apt update
    sudo apt install -y ros-humble-desktop ros-dev-tools \
        python3-colcon-common-extensions ros-humble-rosbag2-storage-mcap
fi

# ---------------------------------------------------------------- 2. python venv
stage "2/6 python venv (.venv)"
if [ -x "$DIR/.venv/bin/python" ]; then
    echo "already exists — skip creation"
else
    uv venv --python /usr/bin/python3.10 --system-site-packages "$DIR/.venv"
fi
uv pip install -p "$DIR/.venv/bin/python" -r "$DIR/requirements.txt"
uv pip install -p "$DIR/.venv/bin/python" --no-deps -e "$DIR/instinct_onboard"

# ------------------------------------------- 3. unitree_ros2 colcon workspace
stage "3/6 unitree_ros2 colcon workspace"
WS="$DIR/unitree_ros2/cyclonedds_ws"
# check for the LAST package built, not install/setup.bash (which appears after the first one)
if [ -d "$WS/install/unitree_hg" ]; then
    echo "already built — skip"
else
    [ -d "$DIR/unitree_ros2" ] || git clone https://github.com/unitreerobotics/unitree_ros2 "$DIR/unitree_ros2"
    cd "$WS/src"
    [ -d rosidl_dds ]      || git clone -b humble          https://github.com/ros2/rosidl_dds
    [ -d cyclonedds ]      || git clone -b releases/0.10.x https://github.com/eclipse-cyclonedds/cyclonedds
    [ -d rmw_cyclonedds ]  || git clone -b humble          https://github.com/ros2/rmw_cyclonedds
    [ -d ros2_numpy ]      || git clone -b humble          https://github.com/Box-Robotics/ros2_numpy
    [ -d tf_transformations ] || git clone                 https://github.com/DLu/tf_transformations
    cd "$WS"
    # cyclonedds must be built with ROS NOT sourced; the rest with ROS sourced.
    # (subshells + set +u because ROS setup scripts reference unset variables)
    ( set +u; colcon build --packages-select cyclonedds )
    ( set +u; source /opt/ros/humble/setup.bash
      colcon build --packages-select rosidl_generator_dds_idl )
    ( set +u; source /opt/ros/humble/setup.bash; source install/setup.bash
      colcon build --packages-select unitree_api unitree_go unitree_hg \
          rmw_cyclonedds_cpp ros2_numpy tf_transformations )
fi

# ---------------------------------------------------------------- 4. crc_module
stage "4/6 crc_module (g1_crc)"
if ls "$DIR/instinct_onboard/scripts"/crc_module*.so > /dev/null 2>&1; then
    echo "already present — skip"
else
    [ -d "$DIR/g1_crc" ] || git clone https://github.com/ZiwenZhuang/g1_crc "$DIR/g1_crc"
    mkdir -p "$DIR/g1_crc/build"
    cd "$DIR/g1_crc/build"
    # pin the interpreter explicitly — cmake otherwise finds whatever python is
    # around (e.g. conda's), producing a .so the 3.10 venv cannot import
    cmake -Dpybind11_DIR="$("$DIR/.venv/bin/python" -m pybind11 --cmakedir)" \
          -DPYTHON_EXECUTABLE="$DIR/.venv/bin/python" \
          -DPython_EXECUTABLE="$DIR/.venv/bin/python" \
          -DPython3_EXECUTABLE="$DIR/.venv/bin/python" ..
    make
    find "$DIR/g1_crc" -name "crc_module*.so" -exec cp {} "$DIR/instinct_onboard/scripts/" \;
    ls "$DIR/instinct_onboard/scripts"/crc_module*.so
fi

# ------------------- 5. sim2sim terrain-generation venv (OPTIONAL)
# Only needed to (re)generate the full training terrain with
# sim2sim/gen_training_terrain.py. Requires an InstinctMJ checkout at the repo
# root (directory or symlink) — not publicly clonable, so we skip if absent.
# The default stairs scene (sim2sim/assets/scene) works without any of this.
stage "5/6 sim2sim terrain venv (optional)"
if [ ! -d "$DIR/InstinctMJ/src/instinct_mj" ]; then
    echo "InstinctMJ checkout not found at $DIR/InstinctMJ — skip"
    echo "(link one to enable full-terrain generation: ln -s /path/to/InstinctMJ $DIR/InstinctMJ)"
elif [ -x "$DIR/.venv-terrain/bin/python" ] \
        && "$DIR/.venv-terrain/bin/python" -c "import mjlab, coacd" 2> /dev/null; then
    echo "already set up — skip"
else
    [ -x "$DIR/.venv-terrain/bin/python" ] || uv venv --python /usr/bin/python3.10 "$DIR/.venv-terrain"
    # cpu-only torch: terrain generation needs no GPU, saves a >2 GB download
    uv pip install -p "$DIR/.venv-terrain/bin/python" \
        --index-url https://download.pytorch.org/whl/cpu torch
    uv pip install -p "$DIR/.venv-terrain/bin/python" \
        mjlab trimesh "mujoco==3.8.1" numpy opencv-python-headless scikit-learn scipy coacd
    echo "generate the terrain with:  .venv-terrain/bin/python sim2sim/gen_training_terrain.py"
fi

# ---------------------------------------------------------------- 6. summary
stage "6/6 done"
echo "Next steps:"
echo "  1. check the NIC name in setup_env.sh (current machine: ip -br link)"
echo "  2. source setup_env.sh"
echo "  3. sanity check against robot:  ros2 topic hz /lowstate   (~1000 Hz)"
echo "  4. dryrun:  python instinct_onboard/scripts/g1_parkour_laptop.py"
echo "  5. sim2sim: python sim2sim/g1_mujoco_bridge.py   (see sim2sim/README.md)"
