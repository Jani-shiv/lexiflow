# Development & Testing Guide

## Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Running Tests
```bash
# Run all unit and integration tests
cargo test --all

# Run memory benchmark suite
cargo test --test memory_benchmark_tests -- --nocapture

# Run concurrency race-condition suite
cargo test --test concurrency_tests -- --nocapture
```

## Code Quality Standards
- No raw keylogging: Minimum required sentence context only.
- Strict memory ceiling: All components must stay within <100MB RAM.
- No remote network socket operations.
