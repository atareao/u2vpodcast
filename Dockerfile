###############################################################################
## Builder
###############################################################################
FROM rust:1.64 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

# Create appuser
ENV USER=app
ENV UID=10001

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

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/u2vpodcast /app/u2vpodcast

###############################################################################
## Final image
###############################################################################
FROM alpine:3.17

RUN apk add --update --no-cache \
            ffmpeg~=5.1 \
            git~=2.38 \
            python3~=3.10 \
            py3-pip~=22.3 && \
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
