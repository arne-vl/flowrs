#!/bin/bash

# Verify rustc
rustc --version || { echo "rustc is not accessible"; exit 1; }

# Verify python
python --version || { echo "python is not accessible"; exit 1; }

# Set alias for easy venv activation
echo alias activate=\'source .venv/bin/activate\' >> ~/.bash_aliases

echo "Setup complete!"
