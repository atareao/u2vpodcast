group "default" {
    targets = ["latest"]
}

variable "REGISTRY_PREFIX" {
    default = "atareao"
}

variable "IMAGE_NAME" {
    default = "u2vpodcast"
}

target "latest" {
    platforms = ["linux/amd64", "linux/arm64"]
    tags = [
        "${REGISTRY_PREFIX}/${IMAGE_NAME}:latest",
        "${REGISTRY_PREFIX}/${IMAGE_NAME}:v0.2.0"
    ]
}
