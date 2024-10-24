# Base rust installation
FROM ubuntu:22.04 as rust

ARG DEBIAN_FRONTEND="noninteractive"

ENV HOME="/root"

# Install build deps
RUN apt-get update -qq && apt-get upgrade -qq && apt-get install -qq \
    build-essential curl pkg-config libssl-dev libudev-dev

# Install Rust
RUN sh -c "$(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs)" -- -y
ENV PATH="${HOME}/.cargo/bin:${PATH}"


# Build server binary
FROM rust as build

WORKDIR /build
COPY Cargo.* .
COPY src src
RUN cargo build --release


# Start server
FROM rust

# Set the workdir
WORKDIR /home/json-rpc-backend

ENV RPC_SERVICE_IP localhost
ENV RPC_SERVICE_PORT 8000
ENV RPC_SERVICE_LOGGING info,warn,error,debug
ENV RPC_SERVICE_WORKERS 1

# Start server
COPY --from=build /build/target/release/json-rpc-backend .
EXPOSE 8080
CMD ["./json-rpc-backend"]
