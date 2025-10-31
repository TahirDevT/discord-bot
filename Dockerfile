# Use the official Rust image as the builder
FROM rust:1.75-slim as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo files and download dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/your_bot_name /usr/local/bin/discord

# Set the command to run your bot
CMD ["discord"]
