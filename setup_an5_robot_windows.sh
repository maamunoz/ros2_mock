#!/bin/bash
# Automated script to configure ROS2 link to AN5 robot
# Opens 7 separate terminal windows (one for each command)
#
# Terminals 1-4 (the ROS2 side) run inside the fr5_ros2 Docker image
# (see Dockerfile/docker-compose.yml) instead of a native colcon build, so
# this box only needs Docker + the AN5 controller reachable at
# 192.168.58.2 -- no local ROS install required. The image already bundles
# fr_ros2, code and rosbridge_server (built by `docker compose build`), so
# nothing extra needs to be added to it for these commands to work.
# `--profile real` is required because fr5-real is profile-gated in
# docker-compose.yml (it targets the real controller, unlike fr5-sim).

echo "Starting AN5 robot ROS2 setup..."

cd ~/ros2_ws || exit 1
echo "Building/checking fr5_ros2 Docker image..."
docker compose build fr5-real

# Terminal 1: ROS2 command server
gnome-terminal --window --title="ROS2 Cmd Server (docker)" -- bash -c "
    cd ~/ros2_ws
    docker compose --profile real run --rm --name an5_ros2_cmd_server fr5-real \
        ros2 run fr_ros2 ros2_cmd_server
    exec bash
"

# Terminal 2: ROSBridge server
gnome-terminal --window --title="ROSBridge Server (docker)" -- bash -c "
    cd ~/ros2_ws
    docker compose --profile real run --rm --name an5_rosbridge fr5-real \
        ros2 launch rosbridge_server rosbridge_websocket_launch.xml
    exec bash
"

# Terminal 3: Publisher subscriber
gnome-terminal --window --title="Publisher Subscriber (docker)" -- bash -c "
    cd ~/ros2_ws
    docker compose --profile real run --rm --name an5_publisher_subscriber fr5-real \
        ros2 run code publisher_subscriber
    exec bash
"

# Terminal 4: API command topic echo
gnome-terminal --window --title="API Command Echo (docker)" -- bash -c "
    cd ~/ros2_ws
    docker compose --profile real run --rm --name an5_api_command_echo fr5-real \
        ros2 topic echo /api_command
    exec bash
"

# Terminal 5: MATLAB inverse kinematics
gnome-terminal --window --title="MATLAB Inverse Kinematics" -- bash -c "
    cd ~/Interfaces-AN5/Interfaz\ AppDesigner\ AN5/
    matlab -nodesktop -r \"run('inverse_kinematics.m')\"
    exec bash
"

# Terminal 6: MATLAB App Designer Simulacion
#gnome-terminal --window --title="MATLAB App Designer" -- bash -c "
#    cd ~/Interfaces-AN5/Interfaz\ AppDesigner\ AN5/
#    matlab -nodesktop -r \"run('Interfaz_App_Designer_Simulacion.m')\"
#    exec bash
#"

# Terminal 7: Unity Interface
gnome-terminal --window --title="Unity Interface" -- bash -c "
    cd ~/Interfaces-AN5/Interfaz\ Unity\ AN5/
    ./Interfaz_AN5.x86_64
    exec bash
"

echo "All 7 terminal windows launched. Check each window for errors."
