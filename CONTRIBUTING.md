# Contributing to Hardware Query

Thank you for your interest in contributing to Hardware Query! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**: `git clone https://github.com/your-username/hardware-query.git`
3. **Add the upstream remote**: `git remote add upstream https://github.com/original-owner/hardware-query.git`
4. **Create a branch** for your feature: `git checkout -b feature/my-feature`

## Development Environment

This project requires Rust stable (1.70+). We recommend using [rustup](https://rustup.rs/) to manage your Rust installation.

## Development Workflow

1. Make your changes in your feature branch
2. Run tests to ensure your changes don't break existing functionality:

   ```bash
   cargo test
   ```

3. Run the benchmarks to ensure performance isn't degraded:

   ```bash
   cargo bench
   ```

4. Format your code using rustfmt:

   ```bash
   cargo fmt
   ```

5. Run clippy to catch common mistakes:

   ```bash
   cargo clippy
   ```

## Pull Request Process

1. Update documentation as needed
2. Add tests for new functionality
3. Ensure all tests pass
4. Submit a pull request against the `main` branch
5. Request review from maintainers

## Coding Standards

- Follow idiomatic Rust practices
- Use `Result<T>` for operations that can fail
- Document public APIs with rustdoc comments
- Follow the existing error handling patterns
- Use platform-conditional compilation for OS-specific code

## Testing Guidelines

- Write unit tests for each module
- Include integration tests for cross-platform compatibility
- Test error handling paths thoroughly
- Mock platform-specific calls in tests when possible

### Testing on Different Platforms

When adding platform-specific code, try to test on the target platform. If you don't have access to a particular platform, please note this in your PR.

## Documentation

- Include comprehensive rustdoc comments
- Provide usage examples for new functionality
- Document platform-specific behavior
- Include performance characteristics where relevant

## Feature Flags

- Use feature flags for optional functionality
- Document feature flags in the README
- Default features should be minimal

## Release Process

1. Bump version in Cargo.toml
2. Update CHANGELOG.md
3. Create a git tag
4. Publish to crates.io (maintainers only)

## License

By contributing, you agree that your contributions will be licensed under the project's MIT OR Apache-2.0 license.
