###############################################################################
## Builder
###############################################################################
FROM rust:1.69 AS builder
LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"
ARG TARGETPLATFORM

# Create appuser
ENV USER=app
ENV UID=10001

ENV RUST_MUSL_CROSS_TARGET=$TARGETPLATFORM

COPY ./platform.sh /platform.sh
RUN /platform.sh && \
    echo $TARGETPLATFORM && \
    cat /.target

#RUN rustup target add x86_64-unknown-linux-musl && \
RUN rustup target add "$(cat /.target)" && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cross build --release --target $(cat /.target) && \
    cp /app/target/$(cat /.target)/release/u2vpodcast /app/u2vpodcast

###############################################################################
## Final image
###############################################################################
FROM --platform=$TARGETPLATFORM alpine:3.18

RUN apk add --update --no-cache \
            ffmpeg~=6.0 \
            git~=2.40 \
            python3~=3.11 \
            py3-pip~=23.1 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*


# Set the work dir
WORKDIR /app

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/u2vpodcast /app/

COPY migrations/ /app/migrations/
COPY templates/ /app/templates/
COPY assets/ /app/assets/

RUN mkdir -p /app/db /app/audios && \
    chown -R app: /app

USER app

RUN python3 -m pip install --user --upgrade git+https://github.com/yt-dlp/yt-dlp.git@release

CMD ["/app/u2vpodcast"]
