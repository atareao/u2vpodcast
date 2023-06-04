user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`
os      := `uname -m`

run:
    cargo run --

watch:
    cargo watch -q -c -w src/ -w assets/ -x run

test:
    #--platform=linux/amd64,linux/arm64/v8,linux/arm/v7 \
    export DOCKER_BUILDKIT=1
    docker buildx build \
        --progress=plain \
        --platform=linux/amd64,linux/arm64/v8 \
        --tag {{user}}/{{name}}:latest \
        --tag  {{user}}/{{name}}:{{version}} \
        --push \
        .

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

create:
    #!/usr/bin/env bash
    set -euxo pipefail
    for ARCH in arm64 arm-v7 arm-v6 amd64; do
        echo "Building ${ARCH}"
        buildah bud \
                --platform linux/${ARCH} \
                --file "Dockerfile.${ARCH}" \
                --tag "docker.io/{{user}}/{{name}}:{{version}}-${ARCH}" \
                --tag "docker.io/{{user}}/{{name}}:latest-${ARCH}" \
                .
    done
    for VERSION in {{version}} latest; do
        echo "Creating manifest for ${VERSION}"
        buildah manifest create "docker.io/{{user}}/{{name}}:${VERSION}" \
                "docker.io/{{user}}/{{name}}:${VERSION}-amd64" \
                "docker.io/{{user}}/{{name}}:${VERSION}-arm64" \
                "docker.io/{{user}}/{{name}}:${VERSION}-arm-v7" \
                "docker.io/{{user}}/{{name}}:${VERSION}-arm-v6"
        echo "Push manifest for ${VERSION}"
        buildah manifest push \
                --all \
                "docker.io/{{user}}/{{name}}:${VERSION}" \
                "docker://docker.io/{{user}}/{{name}}:${VERSION}"
        echo "Delete manifest for ${VERSION}"
        buildah manifest rm \
                "docker.io/{{user}}/{{name}}:${VERSION}"
    done
