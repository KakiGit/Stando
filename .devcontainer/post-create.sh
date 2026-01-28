#!/usr/bin/env bash
set -euo pipefail

rustc --version
cargo --version

# Install myZSH setup
bash -c "$(curl -fsSL https://raw.githubusercontent.com/KakiGit/myZSH/master/setup.sh)"
echo 'export TERM="xterm-256color"' >> /home/vscode/.zshrc
