# ros2_ws — Workspace ROS2 FR5/AN5

Workspace de ROS 2 (Humble) para controlar el brazo robot Fairino FR5/AN5 y
alimentar una interfaz Unity (RosSharp) por `rosbridge_websocket`. Ofrece
tres formas de correr el stack, todas exponiendo la **misma** interfaz
ROS2 (`/FR_ROS_API_service`, `nonrt_state_data`, `/joint_states`,
`current_joint_position`, `current_cartesian_position`), asi que Unity
nunca necesita saber cual de las tres esta corriendo por debajo.

## Paquetes

| Paquete | Tipo | Rol |
|---|---|---|
| `frhal_msgs` | interfaces | `srv/ROSCmdInterface` + `msg/FRState`, compartidos por los demas paquetes |
| `fr_ros2` | C++ | nodo `ros2_cmd_server`: driver del robot real, habla por TCP con el controlador |
| `code` | Python | nodo `publisher_subscriber`: conecta `/api_command` con el servicio del driver y consulta al robot por XML-RPC |
| `an5_mock_sim` | Python | nodo `mock_cmd_server` + launch files: robot simulado, y los wrappers de launch para modo sim/real |

## 1. Nodos ROS2 para el robot FR5 real

`fr_ros2/ros2_cmd_server` es el driver real: se conecta por TCP crudo al
controlador del robot en `192.168.58.2` (puertos `8080`/`8082` para
comandos, `8083` para el feedback de estado) y expone
`/FR_ROS_API_service` (`frhal_msgs/srv/ROSCmdInterface`) mas el topic
`nonrt_state_data` (`frhal_msgs/msg/FRState`) cada 100 ms.

`code/publisher_subscriber` se ubica delante: se suscribe a
`/api_command` (`std_msgs/String`) y reenvia cada mensaje a
`/FR_ROS_API_service`, y por su cuenta consulta al controlador por
XML-RPC (SDK Fairino, puerto `20003`) para publicar
`current_joint_position` / `current_cartesian_position` — los topics que
Unity realmente lee.

A diferencia del mock (que agrupa todo en un unico `sim.launch.py`), el
modo real se corre como nodos separados en terminales separadas — un
proceso por responsabilidad, para poder reiniciar/inspeccionar cada uno
por separado. Requiere el controlador fisico accesible en
`192.168.58.2`.

```bash
# Terminal 1: ROS2 command server (el driver real)
cd ~/ros2_ws
source install/setup.bash
ros2 run fr_ros2 ros2_cmd_server
```

```bash
# Terminal 2: ROSBridge server (puerto 9090, para Unity/RosSharp)
cd ~/ros2_ws
source install/setup.bash
ros2 launch rosbridge_server rosbridge_websocket_launch.xml
```

```bash
# Terminal 3: puente publisher/subscriber (/api_command -> /FR_ROS_API_service)
cd ~/ros2_ws
source install/setup.bash
ros2 run code publisher_subscriber
```

```bash
# Terminal 4 (opcional): ver los comandos que manda Unity
cd ~/ros2_ws
source install/setup.bash
ros2 topic echo /api_command
```

`setup_an5_robot_windows.sh` automatiza la apertura de estas 4 terminales
— enrutadas por Docker en vez de una instalacion nativa, ver seccion 3.

## 2. Mock de ROS2 para simular el robot

`an5_mock_sim/mock_cmd_server` es un reemplazo directo de
`ros2_cmd_server`: sirve el mismo `/FR_ROS_API_service` y
`nonrt_state_data`, y ademas publica `/joint_states`
(`sensor_msgs/JointState`) interpolando hacia los setpoints comandados,
mas `current_joint_position`/`current_cartesian_position` calculados por
cinematica directa — asi Unity ve movimiento sin robot real ni conexion
XML-RPC.

```bash
source install/setup.bash
ros2 launch an5_mock_sim sim.launch.py
```

Argumentos utiles: `joint_states_rate_hz`, `easing`
(`ease_in_out`/`linear`), `initial_joint_positions_deg`,
`include_publisher_subscriber`. Ver
[`src/an5_mock_sim/README.md`](src/an5_mock_sim/README.md) para la
gramatica completa de comandos, las simplificaciones y advertencias
importantes (nunca correr `sim.launch.py` y `real.launch.py` al mismo
tiempo).

## 3. Docker: correr en cualquier lado

Una imagen Docker con ROS 2 Humble y todo el workspace ya compilado, para
correr en cualquier equipo sin instalar ROS ni dependencias a mano.

```bash
docker compose build

# Modo simulado (default, sin robot fisico)
docker compose up fr5-sim

# Modo real, bundleado (requiere el controlador FR5/AN5 en 192.168.58.2)
docker compose --profile real up fr5-real
```

El modo real tambien se puede correr como contenedores separados por
nodo, igual al esquema de una terminal por nodo de la seccion 1 — un
`docker compose run` por nodo en vez del contenedor unico bundleado de
`fr5-sim`:

```bash
docker compose --profile real run --rm --name an5_ros2_cmd_server fr5-real \
    ros2 run fr_ros2 ros2_cmd_server

docker compose --profile real run --rm --name an5_rosbridge fr5-real \
    ros2 launch rosbridge_server rosbridge_websocket_launch.xml

docker compose --profile real run --rm --name an5_publisher_subscriber fr5-real \
    ros2 run code publisher_subscriber

# opcional: ver los comandos que manda Unity
docker compose --profile real run --rm --name an5_api_command_echo fr5-real \
    ros2 topic echo /api_command
```

`setup_an5_robot_windows.sh` automatiza la apertura de estos 4 nodos
como ventanas `gnome-terminal` separadas, cada una corriendo via
`docker compose run` en vez de una instalacion nativa.

Ver [`DOCKER.md`](DOCKER.md) para notas de red (`network_mode: host` vs.
Docker Desktop), como pasar argumentos de launch con
`docker compose run`, e instrucciones de rebuild.

## Build nativo (sin Docker)

```bash
colcon build --symlink-install
source install/setup.bash
```
