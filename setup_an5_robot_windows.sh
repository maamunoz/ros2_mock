#!/bin/bash
# Automated script to configure ROS2 link to AN5 robot
# Opens 7 separate terminal windows (one for each command)

echo "Starting AN5 robot ROS2 setup..."

# Terminal 1: ROS2 command server
gnome-terminal --window --title="ROS2 Cmd Server" -- bash -c "
    cd ~/ros2_ws
    source install/setup.bash
    ros2 run fr_ros2 ros2_cmd_server
    exec bash
"

# Terminal 2: ROSBridge server
gnome-terminal --window --title="ROSBridge Server" -- bash -c "
    cd ~/ros2_ws
    source install/setup.bash
    ros2 launch rosbridge_server rosbridge_websocket_launch.xml
    exec bash
"

# Terminal 3: Publisher subscriber
gnome-terminal --window --title="Publisher Subscriber" -- bash -c "
    cd ~/ros2_ws
    source install/setup.bash
    ros2 run code publisher_subscriber
    exec bash
"

# Terminal 4: API command topic echo
gnome-terminal --window --title="API Command Echo" -- bash -c "
    cd ~/ros2_ws
    source install/setup.bash
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
