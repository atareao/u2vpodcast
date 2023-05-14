user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`
os      := `uname -m`

run:
    cargo run --

watch:
    cargo watch -q -c -w src/ -x run

build:
    echo {{version}}
    echo {{name}}
    podman build -t {{user}}/{{name}}:{{version}} -t {{user}}/{{name}}:latest .

push:
    podman push {{user}}/{{name}}:{{version}}
    podman push {{user}}/{{name}}:latest

build-arm64:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:arm64-{{version}} \
                 --platform linux/arm64 \
                 --file ./Dockerfile.arm64 .
