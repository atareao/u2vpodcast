###############################################################################
## Builder
###############################################################################
FROM rust:latest AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

ARG TARGETARCH=x86_64-unknown-linux-musl
ENV RUST_MUSL_CROSS_TARGET=$TARGETARCH

RUN rustup target add $TARGETARCH && \
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

RUN cargo build --release --target $TARGETARCH && \
    cp /app/target/$TARGETARCH/release/u2vpodcast /app/u2vpodcast

###############################################################################
## Final image
###############################################################################
FROM alpine:3.16

ARG APP=u2vpodcast

RUN apk add --update --no-cache \
            su-exec~=0.2 \
            tzdata~=2022 \
            ffmpeg~=5.0 \
            yt-dlp~=2022 &&\
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Set the work dir
WORKDIR /app

COPY entrypoint.sh /app/
COPY migrations/ /app/migrations/
COPY templates/ /app/templates/
COPY assets/ /app/assets/
# Copy our build
COPY --from=builder /app/$APP /app/

ENTRYPOINT ["/bin/sh", "/app/entrypoint.sh"]
CMD ["/app/u2vpodcast"]
