# Docker

Imagen con ROS 2 Humble + el workspace (`fr_ros2`, `code`, `frhal_msgs`,
`an5_mock_sim`) ya compilado, para correr en cualquier equipo sin instalar
ROS ni dependencias a mano.

## Build

```bash
docker compose build
```

(o `docker build -t fr5_ros2 .` sin compose)

## Modo simulado (default, sin robot fisico)

```bash
docker compose up fr5-sim
```

Levanta `rosbridge_websocket` (puerto **9090**, para Unity/RosSharp) +
`mock_cmd_server` + `publisher_subscriber`, igual que
`ros2 launch an5_mock_sim sim.launch.py` corrido nativo.

## Modo real (requiere el controlador FR5/AN5 en 192.168.58.2)

Bundleado, los tres procesos en un solo contenedor (`real.launch.py`):

```bash
docker compose --profile real up fr5-real
```

### Modo real: nodos separados (un contenedor por nodo)

Igual que corriendolo nativo en 4 terminales separadas, pero cada uno en su
propio contenedor (mismo `network_mode: host`, se descubren entre si sin
problema). Preferible si queres poder reiniciar/inspeccionar un nodo sin
tocar los demas -- es lo que automatiza `setup_an5_robot_windows.sh`:

```bash
# Terminal 1: driver real
docker compose --profile real run --rm --name an5_ros2_cmd_server fr5-real \
    ros2 run fr_ros2 ros2_cmd_server

# Terminal 2: rosbridge (puerto 9090, Unity/RosSharp)
docker compose --profile real run --rm --name an5_rosbridge fr5-real \
    ros2 launch rosbridge_server rosbridge_websocket_launch.xml

# Terminal 3: puente /api_command -> /FR_ROS_API_service
docker compose --profile real run --rm --name an5_publisher_subscriber fr5-real \
    ros2 run code publisher_subscriber

# Terminal 4 (opcional): ver los comandos que manda Unity
docker compose --profile real run --rm --name an5_api_command_echo fr5-real \
    ros2 topic echo /api_command
```

## Docker Desktop (Mac/Windows/Linux)

`docker-compose.yml` usa `network_mode: host`, que en Docker Desktop **no**
expone el puerto al host real (el contenedor lo ve como abierto, pero
`network_mode: host` ahi apunta a la red interna de la VM de Docker
Desktop, no a tu maquina -- confirmado: `rosbridge_websocket` loguea
"started on port 9090" pero `localhost:9090` da connection refused desde
afuera). Usa el compose alternativo `docker-compose.desktop.yml`, que
publica el puerto explicitamente:

```bash
docker compose -f docker-compose.desktop.yml build

docker compose -f docker-compose.desktop.yml up fr5-sim

docker compose -f docker-compose.desktop.yml --profile real up fr5-real
```

Alcanza para que Unity se conecte via rosbridge en `localhost:9090`, pero
a diferencia de `network_mode: host`, nodos ROS2 nativos corriendo fuera
del contenedor no van a poder descubrir los del contenedor (DDS no pasa
por el mapeo de puertos).

Los comandos de "nodos separados" de arriba tambien funcionan en Docker
Desktop agregando `-f docker-compose.desktop.yml` despues de `docker
compose`.

## Notas

- Pasar argumentos de launch (ver tabla en `src/an5_mock_sim/README.md`),
  por ejemplo:

  ```bash
  docker compose run --rm fr5-sim \
    ros2 launch an5_mock_sim sim.launch.py easing:=linear
  ```

- Para reconstruir despues de tocar codigo: `docker compose build` de
  nuevo (no hay bind mount del `src/`, el codigo queda copiado dentro de
  la imagen en el build).
