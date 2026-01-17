#!/bin/bash
# Nebula Benchmark Suite
# Run with: bash bench_all.sh

set -e

echo "🔨 Building Nebula (release)..."
cargo build --release

SPECTER="./target/release/specter"

echo ""
echo "🌌 Nebula Benchmark Suite"
echo "========================="
echo ""

echo "📊 Fibonacci (fib 28):"
time $SPECTER --vm examples/bench_nebula_fib.na

echo ""
echo "📊 Loop (100k iterations):"
time $SPECTER --vm examples/bench_loop_minimal.na

echo ""
echo "📊 Constant Folding:"
time $SPECTER --vm examples/test_const_fold.na

echo ""
echo "✨ All benchmarks complete!"
