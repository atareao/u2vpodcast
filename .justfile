user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`
os      := `uname -m`

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{os}}-{{version}} .

tag:
    docker tag {{user}}/{{name}}:{{os}}-{{version}} {{user}}/{{name}}:{{os}}-latest

push:
    docker push {{user}}/{{name}}:{{os}}-{{version}}
    docker push {{user}}/{{name}}:{{os}}-latest

build-arm64:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:arm64-{{version}} \
                 --platform linux/arm64 \
                 --file ./Dockerfile.arm64 .

tag-arm64:
    docker tag {{user}}/{{name}}:arm64-{{version}} {{user}}/{{name}}:arm64-latest

push-arm64:
    docker push {{user}}/{{name}}:arm64-{{version}}
    docker push {{user}}/{{name}}:arm64-latest

buildx-arm64:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    docker buildx build \
           --push \
           --platform linux/arm64 \
           --file ./Dockerfile.arm64 \
           --tag {{user}}/{{name}}:arm64-{{version}} \
           .

buildx-amd64:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    docker buildx build \
           --push \
           --platform linux/amd64 \
           --tag {{user}}/{{name}}:{{version}} \
           --file Dockerfile.amd64 \
           .

run:
    docker run --rm \
               --init \
               --name croni \
               --init \
               --env_file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{os}}-{{version}}

sh:
    docker run --rm \
               -it \
               --name croni \
               --init \
               --env-file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{os}}-{{version}} \
               sh

