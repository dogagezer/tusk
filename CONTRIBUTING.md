# Contributing to TUSK

Thanks for your interest in contributing! TUSK is participating in
Hacktoberfest, and contributions of all sizes are welcome.

## Getting started

1. Fork the repository and create a feature branch:
   ```bash
   git clone https://github.com/YOUR_USERNAME/tusk.git
   cd tusk
   git checkout -b feature-my-change
   ```
2. Make sure Rust tooling is installed:
   ```bash
   cargo --version
   rustc --version
   ```
3. Build and test locally:
   ```bash
   cargo build
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

## Guidelines

- Keep changes focused. Small pull requests are easier to review.
- Follow the existing code style (run `cargo fmt`).
- Add tests when you change behavior.
- Update the README or help text if you add or change a command.
- Be respectful and follow the Code of Conduct.

## Hacktoberfest

- Issues labeled `hacktoberfest` and `good first issue` are great places to start.
- Please open an issue or comment before starting large changes.
- Low-quality or spammy PRs may be marked invalid, per Hacktoberfest rules.
- Maintainer review is required before merge.

## Pull request checklist

- [ ] Builds successfully (`cargo build`)
- [ ] Tests pass (`cargo test`)
- [ ] Formatted (`cargo fmt`)
- [ ] Documentation updated where relevant
```