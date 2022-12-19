###############################################################################
## Builder
###############################################################################
FROM --platform=$BUILDPLATFORM rust:latest AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

#ARG TARGET=x86_64-unknown-linux-musl
ARG TARGETARCH
ARG OPENSSL_LIB_DIR
WORKDIR /app
COPY platform.sh .
RUN ./platform.sh

ENV RUST_MUSL_CROSS_TARGET=$TARGETARCH
ENV OPENSSL_LIB_DIR=$OPENSSL_LIB_DIR
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

RUN echo $OPENSSL_LIB_DIR && \
    rustup component add rustfmt && \
    rustup target add "$(cat /.platform)" && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        gcc-arm* \
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        musl-dev \
        pkg-config \
        libssl-dev \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*


COPY Cargo.toml .
COPY Cargo.lock .
#COPY .cargo/config .cargo/config

COPY src src

RUN cargo build --release --target $(cat /.platform) && \
    cp /app/target/$(cat /.platform)/release/u2vpodcast /app/u2vpodcast

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
# Copy the user

# Set the work dir
WORKDIR /app

COPY entrypoint.sh /app/
COPY migrations/ /app/migrations/
COPY templates/ /app/templates/
# Copy our build
#COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/$APP /app/
COPY --from=builder /app/$APP /app/

ENTRYPOINT ["/bin/sh", "/app/entrypoint.sh"]
CMD ["/app/u2vpodcast"]
