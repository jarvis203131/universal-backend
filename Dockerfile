# Stage 1: Build
FROM rust:1.80-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create workspace directory
WORKDIR /usr/src/app

# Copy the entire workspace
COPY . .

# Build the gateway binary in release mode
RUN cargo build --release -p gateway

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies (openssl is required for sqlx/crypto)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/gateway /app/gateway

# Expose the API port
EXPOSE 8080

# Set the execution command
CMD ["./gateway"]
