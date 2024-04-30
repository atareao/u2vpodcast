user    := "atareao"
name    := `toml get Cargo.toml package.name --raw`
version := `toml get Cargo.toml package.version --raw`
os      := `uname -m`

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} \
                 -t {{user}}/{{name}}:latest \
                 .

push:
    docker push {{user}}/{{name}}:{{version}}
    # docker push {{user}}/{{name}}:latest

build-arm64:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:arm64-{{version}} \
                 --platform linux/arm64 \
                 --file ./Dockerfile.arm64 .

up:
    docker compose -f ./docker-compose.standalone.yml up -d --force-recreate
down:
    docker compose -f ./docker-compose.standalone.yml down
log:
    docker compose -f ./docker-compose.standalone.yml logs
logs:
    docker compose -f ./docker-compose.standalone.yml logs -f
enter:
    docker compose -f ./docker-compose.standalone.yml exec u2vpodcast sh
