FROM rust:1.86

WORKDIR /app

# Install Solana CLI (for build-sbf) - fix the missing space
RUN apt-get update && apt-get install -y curl ca-certificates pkg-config libssl-dev libsystemd-dev && \
    sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Default command to build (this will be overridden by docker-compose)
CMD ["cargo", "build-sbf", "--manifest-path", "Cargo.toml", "--sbf-out-dir", "/app/target/deploy"]
