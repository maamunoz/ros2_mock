# ros2_ws — FR5/AN5 ROS2 workspace

ROS 2 (Humble) workspace for controlling the Fairino FR5/AN5 robot arm and
driving a Unity (RosSharp) front-end over `rosbridge_websocket`. It offers
three ways to run the stack, all exposing the **same** ROS2 interface
(`/FR_ROS_API_service`, `nonrt_state_data`, `/joint_states`,
`current_joint_position`, `current_cartesian_position`) so Unity never needs
to know which one is running underneath.

## Packages

| Package | Type | Role |
|---|---|---|
| `frhal_msgs` | interfaces | `srv/ROSCmdInterface` + `msg/FRState`, shared by the other packages |
| `fr_ros2` | C++ | `ros2_cmd_server` node: real robot driver, talks TCP to the controller |
| `code` | Python | `publisher_subscriber` node: bridges `/api_command` to the driver's service and polls the robot via XML-RPC |
| `an5_mock_sim` | Python | `mock_cmd_server` node + launch files: simulated robot, and the sim/real launch wrappers |

## 1. ROS2 nodes for the real FR5 robot

`fr_ros2/ros2_cmd_server` is the real driver: it connects over raw TCP to
the robot controller at `192.168.58.2` (ports `8080`/`8082` for commands,
`8083` for state feedback) and exposes `/FR_ROS_API_service`
(`frhal_msgs/srv/ROSCmdInterface`) plus the `nonrt_state_data`
(`frhal_msgs/msg/FRState`) topic at 100 ms.

`code/publisher_subscriber` sits in front of it: it subscribes to
`/api_command` (`std_msgs/String`) and forwards each message to
`/FR_ROS_API_service`, and separately polls the controller via XML-RPC
(Fairino SDK, port `20003`) to publish `current_joint_position` /
`current_cartesian_position` — the topics Unity actually reads.

Unlike the mock (which bundles everything into a single `sim.launch.py`),
real mode is run as separate nodes in separate terminals — one process per
concern, so each can be restarted/inspected independently. Requires the
physical controller reachable at `192.168.58.2`.

```bash
# Terminal 1: ROS2 command server (the real driver)
cd ~/ros2_ws
source install/setup.bash
ros2 run fr_ros2 ros2_cmd_server
```

```bash
# Terminal 2: ROSBridge server (port 9090, for Unity/RosSharp)
cd ~/ros2_ws
source install/setup.bash
ros2 launch rosbridge_server rosbridge_websocket_launch.xml
```

```bash
# Terminal 3: Publisher/subscriber bridge (/api_command -> /FR_ROS_API_service)
cd ~/ros2_ws
source install/setup.bash
ros2 run code publisher_subscriber
```

```bash
# Terminal 4 (optional): watch commands Unity is sending
cd ~/ros2_ws
source install/setup.bash
ros2 topic echo /api_command
```

`setup_an5_robot_windows.sh` automates opening terminals 1-4 (plus MATLAB
and Unity) as separate `gnome-terminal` windows.

## 2. ROS2 mock to simulate the robot

`an5_mock_sim/mock_cmd_server` is a drop-in replacement for
`ros2_cmd_server`: it serves the same `/FR_ROS_API_service` and
`nonrt_state_data`, and additionally publishes `/joint_states`
(`sensor_msgs/JointState`) by interpolating toward commanded setpoints, plus
`current_joint_position`/`current_cartesian_position` computed via forward
kinematics — so Unity gets movement without any real robot or XML-RPC
connection.

```bash
source install/setup.bash
ros2 launch an5_mock_sim sim.launch.py
```

Useful arguments: `joint_states_rate_hz`, `easing` (`ease_in_out`/`linear`),
`initial_joint_positions_deg`, `include_publisher_subscriber`. See
[`src/an5_mock_sim/README.md`](src/an5_mock_sim/README.md) for the full
command grammar, simplifications, and important warnings (never run
`sim.launch.py` and `real.launch.py` at the same time).

## 3. Docker: run it anywhere

A Docker image with ROS 2 Humble and the whole workspace pre-built, for
running on any machine without installing ROS or dependencies by hand.

```bash
docker compose build

# Simulated mode (default, no physical robot needed)
docker compose up fr5-sim

# Real mode (requires the FR5/AN5 controller at 192.168.58.2)
docker compose --profile real up fr5-real
```

See [`DOCKER.md`](DOCKER.md) for networking notes (`network_mode: host`
vs. Docker Desktop), passing launch arguments through `docker compose run`,
and rebuild instructions.

## Native build (without Docker)

```bash
colcon build --symlink-install
source install/setup.bash
```
