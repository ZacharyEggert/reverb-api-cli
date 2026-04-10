#!/usr/bin/env bash
# Generate an HTML code coverage report using cargo-llvm-cov
set -euo pipefail

cargo llvm-cov --all-features --workspace --html --output-dir coverage

echo "Coverage report: coverage/html/index.html"
