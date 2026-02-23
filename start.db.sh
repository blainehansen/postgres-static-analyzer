# podman stop --ignore postgres
# podman rm --ignore postgres

podman run --name postgres --rm -e POSTGRES_PASSWORD=devpassword -e POSTGRES_USER=devuser -e POSTGRES_DB=devdb -p 5432:5432 docker.io/library/postgres:17
