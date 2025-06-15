# Build stage
FROM rust:1.86 AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    pkg-config \
    libssl-dev \
    libsystemd-dev \
    && rm -rf /var/lib/apt/lists/* \
    && sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Copy only the necessary files for building
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Debug: Show current directory contents
RUN pwd && ls -la

# Build the Solana program with verbose output
RUN cargo build-sbf --manifest-path Cargo.toml --sbf-out-dir /app/target/deploy -v

# Debug: Show build output directory
RUN ls -la /app/target/deploy/

# Echo success
RUN echo "Solana program built successfully"

# Keep the container running
CMD ["tail", "-f", "/dev/null"]
