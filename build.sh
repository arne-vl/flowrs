#!/bin/bash

# Exit script on any error
set -e

# Change to the directory containing Cargo.toml
cd "flowrs"

# Check if maturin is installed
if ! command -v maturin &> /dev/null; then
    echo "maturin is not installed. Installing maturin..."
    pip install maturin
fi

# Build the Python package
echo "Building the Python package using maturin..."
maturin build --release

# Find the generated wheel file
WHEEL_FILE=$(ls target/wheels/*.whl | head -n 1)

if [ -z "$WHEEL_FILE" ]; then
    echo "Error: No wheel file found in target/wheels/"
    exit 1
fi

# Install the package
echo "Installing the package: $WHEEL_FILE..."
pip install --force-reinstall "$WHEEL_FILE"

echo "Package installed successfully!"
