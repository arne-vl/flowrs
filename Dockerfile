# Use a base image with Python preinstalled
FROM python:3.13.1-slim

# Install Rust and dependencies for maturin
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Add Rust binaries to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install maturin
RUN pip install --no-cache-dir maturin

# Verify installations
RUN python --version && rustc --version && cargo --version && maturin --version

# Set up a working directory
WORKDIR /workspace
