FROM rust:1.82.0-slim

# Set working directory in the container
WORKDIR /app

# Install dependencies for OpenSSL and pkg-config
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy all files into the container
COPY . .

# Install cargo-watch for automatic file change monitoring (optional)
RUN cargo install cargo-watch

# Install sea-orm-cli for migrations
RUN cargo install sea-orm-cli

# Build the application in release mode
RUN cargo build --release

# Expose the port that the application will use
EXPOSE 8080

# Command to run the application, including SeaORM migrations
CMD sea-orm-cli migrate up && ./target/release/example-payment-gateway


