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

```bash
docker compose --profile real up fr5-real
```

## Notas

- Los servicios usan `network_mode: host` (Linux) para que el contenedor
  vea la red del controlador real y para que la comunicacion ROS2/DDS no
  tenga problemas con NAT. En Docker Desktop (Mac/Windows), donde
  `network_mode: host` no aplica igual, sacar esa linea y usar en cambio
  `ports: ["9090:9090"]` -- alcanza para que Unity se conecte via
  rosbridge, pero nodos ROS2 nativos fuera del contenedor no van a poder
  descubrir los del contenedor.
- Pasar argumentos de launch (ver tabla en `src/an5_mock_sim/README.md`),
  por ejemplo:

  ```bash
  docker compose run --rm fr5-sim \
    ros2 launch an5_mock_sim sim.launch.py easing:=linear
  ```

- Para reconstruir despues de tocar codigo: `docker compose build` de
  nuevo (no hay bind mount del `src/`, el codigo queda copiado dentro de
  la imagen en el build).
