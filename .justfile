user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`
os      := `uname -m`

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} -t {{user}}/{{name}}:latest .

push:
    docker push --all-tags {{user}}/{{name}}

build-arm64:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:arm64-{{version}} \
                 --platform linux/arm64 \
                 --file ./Dockerfile.arm64 .
