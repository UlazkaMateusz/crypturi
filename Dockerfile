# Use the official Rust image for building the project
FROM rust:1.86 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml, Cargo.lock, and source code to the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the binary in release mode
RUN cargo build --release

# Use a minimal runtime image (Debian Bookworm has OpenSSL 3.x)
FROM debian:bookworm-slim

# Install necessary runtime dependencies, including OpenSSL
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set a working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/crypturi /app/crypturi
RUN chmod +x /app/crypturi

# Copy static files (e.g., public folder for your static assets)
COPY public ./public
COPY templates ./templates

# Expose the port your application listens on
EXPOSE 8040

# Run the application
CMD ["./crypturi"]
