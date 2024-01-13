# Build stage
FROM rust:1.74 as builder
WORKDIR /usr/src/ello
RUN cargo init
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# Run stage
FROM rust:1.74-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/ello/target/release/ello /usr/local/bin/ello
EXPOSE 8080
CMD ["ello"]