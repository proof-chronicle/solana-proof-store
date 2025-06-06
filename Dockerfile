FROM rust:1.86

WORKDIR /app

# Install Solana CLI (for build-sbf) - fix the missing space
RUN apt-get update && apt-get install -y curl ca-certificates pkg-config libssl-dev libsystemd-dev && \
    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Copy source code
COPY . .

# Build the Solana program during image creation
RUN cargo build-sbf --manifest-path Cargo.toml --sbf-out-dir /app/target/deploy

# Use a no-op command since the build is already done
CMD ["echo", "Solana program built successfully"]
