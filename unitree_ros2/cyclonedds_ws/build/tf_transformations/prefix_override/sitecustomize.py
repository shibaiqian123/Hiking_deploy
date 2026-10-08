import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/robot/Hiking_deploy/unitree_ros2/cyclonedds_ws/install/tf_transformations'
