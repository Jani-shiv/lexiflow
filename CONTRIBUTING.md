# Contributing to LexiFlow

Thank you for your interest in contributing to **LexiFlow**! We welcome bug reports, feature requests, documentation improvements, and code contributions.

---

## Code of Conduct

Please review and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md) in all project interactions.

---

## Development Setup

### Prerequisites
- **Rust Toolchain**: Stable (1.75.0 or newer recommended)
- **Cargo**: Included with Rust

### Building the Project
```bash
# Clone the repository
git clone https://github.com/Jani-shiv/lexiflow.git
cd lexiflow

# Run all tests
cargo test --all

# Build optimized release binary
cargo build --release

# Run benchmarks
cargo run --release -- --benchmark
```

---

## Contribution Guidelines

1. **Fork & Branch**:
   - Create a feature branch with a descriptive name: `git checkout -b feat/my-new-rule` or `fix/issue-123`.

2. **Code Standards**:
   - Follow standard Rust idiomatic conventions (`cargo fmt` and `cargo clippy`).
   - Maintain the strict memory budget: any new feature or data structure must not breach the `< 100 MB` RSS threshold.
   - Maintain 100% offline isolation: do not introduce external networking crates or cloud API dependencies.

3. **Testing**:
   - Add unit/integration tests for every new grammar rule, abbreviation, or security filter in `tests/`.
   - Ensure all 57+ existing tests continue to pass with `cargo test --all`.

4. **Pull Requests**:
   - Open a PR targeting the `main` branch.
   - Provide a clear summary of changes, benchmark impacts (if any), and testing evidence.
