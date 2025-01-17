# Use a base image with Python preinstalled
FROM mcr.microsoft.com/devcontainers/python:3.13

# Install Rust and dependencies for maturin
RUN apt-get update && apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    rustup default stable

# Add Rust binaries to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a virtual environment
RUN python -m venv .venv

# Ensure venv is activated in every terminal session
RUN echo "source /workspaces/.venv/bin/activate" >> ~/.bashrc

# Install maturin globally in the venv
RUN . .venv/bin/activate && pip install maturin

# Set up a working directory
WORKDIR /workspace
