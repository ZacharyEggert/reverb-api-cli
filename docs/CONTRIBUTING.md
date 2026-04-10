# Contributing to Reverb CLI

## Getting Started

1. Fork the repository and clone your fork.
2. Install Rust: https://rustup.rs
3. Install Node.js >= 18 and pnpm: `npm install -g pnpm`
4. Install git hooks: `pnpm prepare` (requires lefthook)
5. Copy `.env.example` to `.env` and set `REVERB_API_KEY`.

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## Submitting Changes

1. Create a branch: `git checkout -b your-feature`
2. Make your changes with tests.
3. Add a changeset: create `.changeset/<descriptive-name>.md` (see AGENTS.md).
4. Open a pull request against `main`.

## Guidelines

- Follow the rules in [AGENTS.md](../AGENTS.md).
- Every PR must include a changeset file.
- New code paths must have test coverage.
- Do not commit `.env` or any file containing API keys.
