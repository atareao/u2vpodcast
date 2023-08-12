###############################################################################
## Builder
###############################################################################
FROM rust:1.70 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN rustup target add x86_64-unknown-linux-musl && \
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

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/u2vpodcast /app/u2vpodcast

###############################################################################
## Final image
###############################################################################
FROM alpine:3.18

ENV USER=app
ENV UID=10001

RUN apk add --update --no-cache \
            ffmpeg~=6.0 \
            git~=2.40 \
            python3~=3.11 \
            py3-pip~=23.1 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/u2vpodcast /app/

COPY migrations/ /app/migrations/
COPY templates/ /app/templates/
COPY assets/ /app/assets/

# Create the user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}" && \
    mkdir -p /app/db /app/audios && \
    chown -R app: /app

# Set the work dir
WORKDIR /app
USER app

RUN python3 -m pip install --user --upgrade git+https://github.com/yt-dlp/yt-dlp.git@release

CMD ["/app/u2vpodcast"]
