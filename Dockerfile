FROM rust:slim-buster as builder

WORKDIR /usr/src/brmq 

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get -y install ca-certificates openssl libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/brmq /usr/local/bin/brmq

RUN addgroup -g 1000 brmq

RUN adduser -D -s /bin/sh -u 1000 -G brmq brmq

RUN chown brmq:brmq brmq

USER brmq

EXPOSE 8080

CMD ["brmq"]

# RUN apt-get install musl-tools -y && apt-get -y install ca-certificates libssl-dev

# RUN rustup target add x86_64-unknown-linux-musl

# WORKDIR /src/brmq

# COPY Cargo.toml Cargo.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/brmq*

# COPY ./ ./

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# # ------------------------------------------------------------------------------
# # Final Stage
# # ------------------------------------------------------------------------------

# FROM alpine:latest

# 

# 

# WORKDIR /home/brmq/bin/

# COPY --from=cargo-build /usr/src/brmq/target/x86_64-unknown-linux-musl/release/brmq .