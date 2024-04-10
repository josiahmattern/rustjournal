#!/bin/bash

# Build the project
cargo build --release

# Copy the executable
cp target/release/journal ~/.local/bin/

# Make it executable
chmod +x ~/.local/bin/journal

echo "Installation completed!"
