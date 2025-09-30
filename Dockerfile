# Use the official Rust image as base
FROM rust:1.90-slim

# Install system dependencies including Stellar CLI requirements
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    make \
    libdbus-1-3 \
    libgtk-3-0 \
    libx11-6 \
    libx11-xcb1 \
    libxcb1 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxrandr2 \
    libxrender1 \
    libxss1 \
    libxtst6 \
    ca-certificates \
    fonts-liberation \
    libappindicator3-1 \
    libasound2 \
    libatk-bridge2.0-0 \
    libdrm2 \
    libgtk-3-0 \
    libnspr4 \
    libnss3 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libxss1 \
    libxtst6 \
    xdg-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Stellar CLI
RUN curl -sSL https://github.com/stellar/stellar-cli/releases/latest/download/stellar-cli-23.1.3-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C /usr/local/bin

RUN rustup target add wasm32v1-none

# Set working directory
WORKDIR /workspace

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY contracts/ ./contracts/

# Create a non-root user for security
RUN useradd -m -s /bin/bash developer && \
    chown -R developer:developer /workspace

USER developer

# Set environment variables
ENV RUST_BACKTRACE=1
ENV CARGO_TARGET_DIR=/workspace/target

# Default command
CMD ["/bin/bash"]
