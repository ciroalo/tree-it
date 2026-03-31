#!/usr/bin/env bash
set -e

echo "Installing tree-it..."
cargo install --path .

echo
echo "Installed successfully."
echo "Try: "
echo "  tree-it --help"