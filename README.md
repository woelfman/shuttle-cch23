# Shuttle's Christmas Code Hunt

## Start the database

```sh
podman system service --time=0 unix:///tmp/podman.sock &
podman run --rm --name pg -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres:15 &
```

## Prepare sqlx

```sh
cargo sqlx migrate run
cargo sqlx prepare
```

## Run shuttle locally

```sh
cargo shuttle run
```