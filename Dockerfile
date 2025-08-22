FROM node:20-alpine AS frontend-builder

WORKDIR /app/web

COPY web/package.json web/pnpm-lock.yaml ./

RUN npm install -g pnpm && pnpm install

COPY web/ ./

RUN pnpm run build

FROM rust:1.89-slim-bookworm AS backend-builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_DIR=/app/target
ENV RUSTFLAGS="--cfg tokio_unstable"

COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY config/Cargo.toml config/
COPY core/Cargo.toml core/
COPY database/Cargo.toml database/
COPY error/Cargo.toml error/
COPY git/Cargo.toml git/
COPY session/Cargo.toml session/
COPY migration/Cargo.toml migration/
COPY web/Cargo.toml web/

RUN mkdir -p api/src && touch api/src/lib.rs \
    && mkdir -p config/src && touch config/src/lib.rs \
    && mkdir -p core/src && touch core/src/lib.rs \
    && mkdir -p database/src && touch database/src/lib.rs \
    && mkdir -p error/src && touch error/src/lib.rs \
    && mkdir -p git/src && touch git/src/lib.rs \
    && mkdir -p session/src && touch session/src/lib.rs \
    && mkdir -p migration/src && touch migration/src/lib.rs \
    && mkdir -p web/src && touch web/src/lib.rs \
    && mkdir -p web/embed && touch web/embed/lib.rs \
    && mkdir -p bin && echo 'fn main() {}' > bin/main.rs

RUN cargo fetch

COPY api/src/ api/src/
COPY config/src/ config/src/
COPY core/src/ core/src/
COPY database/src/ database/src/
COPY error/src/ error/src/
COPY git/src/ git/src/
COPY session/src/ session/src/
COPY migration/src/ migration/src/
COPY web/src/ web/src/
COPY web/embed/ web/embed/
COPY bin/ bin/

COPY --from=frontend-builder /app/web/dist /app/web/dist

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash appuser

RUN mkdir -p /app/data/repo /var/run/sshd

WORKDIR /app

COPY --from=backend-builder /app/target/release/jzfs /app/

COPY config.toml /app/

RUN chown -R root:root /app/data

ENV RUST_LOG=info
ENV DEV=false

USER root

EXPOSE 7070 30322

CMD ["./jzfs","run"]
