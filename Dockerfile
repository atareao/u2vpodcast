###############################################################################
## Builder
###############################################################################
FROM rust:1.70 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

ARG TARGET=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGET
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

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
        libssl-dev \
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
FROM --platform=$TARGETPLATFORM alpine:3.18

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
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
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
    chmod 700 /app/u2vpodcast && \
    mkdir -p /app/{db,audios} && \
    chown -R app:app /app

# Set the work dir
WORKDIR /app
USER app

# Install and update yt-dlp
RUN python3 -m pip install --user --upgrade git+https://github.com/yt-dlp/yt-dlp.git@release

CMD ["/app/u2vpodcast"]
