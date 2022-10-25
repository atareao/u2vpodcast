author  := "atareao"
name    := `grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml`
version := `grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml`

build:
    docker build -t "{{author}}/{{name}}:v{{version}}" .

latest:
    docker image tag "{{author}}/{{name}}:v{{version}}" "{{author}}/{{name}}":latest
    docker push "{{author}}/{{name}}:latest"

push:
    docker push "{{author}}/{{name}}:v{{version}}"

run:
    docker run -it --rm --init --env-file .env --name "{{name}}" "{{author}}/{{name}}:v{{version}}"
