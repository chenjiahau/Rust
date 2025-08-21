FROM rust:1.83-slim AS builder

# Set working directory
WORKDIR /app

# Install build tools
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev libpq-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy the full source into the builder stage
COPY . .

# Compile the Rust project
RUN cargo build --release

# ---------------------
# Runtime stage
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/accounting-api /usr/local/bin/accounting-api

# Copy the JSON file your app needs
COPY --from=builder /app/data/spending_categories.json /app/data/spending_categories.json

# Set working directory for runtime
WORKDIR /app

# Run the binary
CMD ["/usr/local/bin/accounting-api"]