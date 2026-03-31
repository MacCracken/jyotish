#!/usr/bin/env bash
# Run criterion benchmarks and archive the results.
set -euo pipefail

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
RESULTS_DIR="bench-history"
mkdir -p "$RESULTS_DIR"

echo "Running benchmarks at $TIMESTAMP ..."
cargo bench --all-features 2>&1 | tee "$RESULTS_DIR/$TIMESTAMP.txt"
echo "Results saved to $RESULTS_DIR/$TIMESTAMP.txt"
