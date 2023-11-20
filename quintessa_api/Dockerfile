# Stage 1: Building
FROM rust:latest as builder

# Install MUSL
RUN apt-get update && \
    apt-get install -y musl-tools && \
    ln -s /usr/bin/musl-gcc /usr/local/bin/musl-gcc && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY . .

# Will build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release --target x86_64-unknown-linux-musl && mv ./target/x86_64-unknown-linux-musl/release/api ./api

# Runtime image
FROM alpine:latest

# Run as "app" user
RUN addgroup -S app && adduser -S app -G app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/api /app/api

# Run the app
CMD ./api
