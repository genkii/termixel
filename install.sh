#!/usr/bin/env bash

set -e

URL="https://github.com/genkii/termixel/releases/download/v0.1.0/termixel"
INSTALL_DIR="$HOME/.local/bin"
BINARY="$INSTALL_DIR/termixel"

echo "Installing Termixel..."

mkdir -p "$INSTALL_DIR"

curl -fL "$URL" -o "$BINARY"

chmod +x "$BINARY"

echo
echo "Termixel has been installed to:"
echo "$BINARY"
echo
echo "To add ~/.local/bin to your PATH, add this to your shell config:"
echo
echo 'export PATH="$HOME/.local/bin:$PATH"'
echo
echo "For Bash, add it to ~/.bashrc"
echo "For Zsh, add it to ~/.zshrc"
echo "For Fish, add ~/.local/bin to your PATH using:"
echo "  fish_add_path ~/.local/bin"
echo
echo "Then restart your shell or reload your config."
