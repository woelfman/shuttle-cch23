# Shuttle's Christmas Code Hunt

## Install shuttle

```sh
cargo install shuttle
```

## Setup shuttle for podman (skip if using docker).

```sh
podman system service --time=0 unix:///tmp/podman.sock &
export DOCKER_HOST=unix:///tmp/podman.sock
```

## Run shuttle locally

```sh
shuttle run
```
