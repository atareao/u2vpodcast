###############################################################################
## Backend builder
###############################################################################
FROM rust:alpine3.19 AS backend_builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN apk add --update --no-cache \
            musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release && \
    cp /app/target/release/u2vpodcast /app/u2vpodcast

###############################################################################
## Frontend builder
###############################################################################
FROM node:20-alpine AS frontend_base
ENV PNPM_HOME="/pnpm" \
    PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY ./frontend/ /app
WORKDIR /app

FROM frontend_base AS frontend_deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store \
    pnpm install --prod --frozen-lockfile

FROM frontend_base AS frontend_builder
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install \
    --frozen-lockfile && \
    pnpm run build

###############################################################################
## Final image
###############################################################################
FROM alpine:3.19

ENV USER=app
ENV UID=10001

RUN apk add --update --no-cache \
            ffmpeg~=6.1 \
            git~=2.43 \
            sqlite~=3.44 \
            python3~=3.11 \
            py3-pip~=23.3 && \
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy from backend_builder
COPY --from=backend_builder /app/u2vpodcast /app/
COPY --from=frontend_deps /app/node_modules /app/html/node_modules
COPY --from=frontend_builder /app/build /app/html

COPY migrations/ /app/migrations/

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

RUN python3 -m pip install \
            --user \
            --upgrade \
            --break-system-packages \
            yt-dlp
            #git+https://github.com/yt-dlp/yt-dlp.git@release

CMD ["/app/u2vpodcast"]
