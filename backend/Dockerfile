# Start from the official Rust image
FROM rust:1.74 as builder

WORKDIR /usr/src/ello

# Create a new empty shell project
RUN cargo init

# Copy over your source code
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

# Build the application
RUN cargo build --release

# Start release stage
FROM rust:1.74-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/ello/target/release/ello /usr/local/bin/ello

# Expose port 8080
EXPOSE 8080

# Set the startup command
CMD ["ello"]