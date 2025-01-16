#!/bin/bash

# Virtual environment directory
VENV_DIR=".venv"

# Function to activate virtual environment
activate_venv() {
    # shellcheck source=/dev/null
    source "$VENV_DIR/bin/activate"
    echo "Activated virtual environment: $VENV_DIR"
}

# Check if a virtual environment exists
if [ ! -d "$VENV_DIR" ]; then
    echo "No virtual environment found. Creating one in $VENV_DIR..."
    python3 -m venv "$VENV_DIR"
    activate_venv
    echo "Installing pip and maturin in the virtual environment..."
    pip install --upgrade pip maturin
else
    echo "Virtual environment found. Activating it..."
    activate_venv
fi

# Check if flowrs is already installed in the virtual environment
if pip show flowrs &> /dev/null; then
    echo "flowrs is already installed in the virtual environment. Removing it..."
    pip uninstall -y flowrs
fi

# Change to the directory containing Cargo.toml
cd "flowrs"

# Ensure maturin is installed in the virtual environment
if ! pip show maturin &> /dev/null; then
    echo "maturin is not installed in the virtual environment. Installing maturin..."
    pip install maturin
fi

# Build and install the package in development mode
echo "Building and installing the Python package using maturin develop..."
maturin develop

echo "Package installed successfully in development mode!"
