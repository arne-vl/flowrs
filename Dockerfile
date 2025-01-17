# Use a base image with Python preinstalled
FROM mcr.microsoft.com/devcontainers/python:3.13

# Install dependencies for maturin and Rust for vscode user
RUN sudo apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    && sudo apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Switch to vscode user and install Rust for vscode user
USER vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set up Rust
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Create a virtual environment with maturin inside /workspace
WORKDIR /workspace
RUN python -m venv .venv \
    && . ./.venv/bin/activate && pip install maturin

# Verify installations
RUN python --version && rustc --version && cargo --version

# Set up a working directory
WORKDIR /workspace
