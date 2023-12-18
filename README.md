# Shuttle's Christmas Code Hunt

## Start the database

```sh
podman system service --time=0 unix:///tmp/podman.sock &
podman run --rm --name pg -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres:15 &

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql
```

## Prepare sqlx

```sh
cargo sqlx migrate run
cargo sqlx prepare
```

## Run shuttle locally

```sh
export DOCKER_HOST=unix:///tmp/podman.sock
cargo shuttle run
```