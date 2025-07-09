sea := require("sea")
docker := require("docker")

check-docker-perm:
    @docker ps > /dev/null || (echo "Error: Docker permissions not set up properly. Try adding user to docker group or use sudo"; exit 1)

@generate-entities crate: check-docker-perm
    docker run -d --name yral-neon-postgres-ephemeral -p 12345:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres:17
    sleep 3 # gives the postgres time to get online
    sea migrate --database-url 'postgresql://postgres@localhost:12345/postgres' fresh -d {{crate}}
    sea generate entity --database-url 'postgresql://postgres@localhost:12345/postgres' -o {{clean(crate / "src" / "entities")}}
    docker rm -f yral-neon-postgres-ephemeral
