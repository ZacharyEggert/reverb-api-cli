# AGENTS.md

## Project Overview

`revcli` is a Rust CLI tool for interacting with the Reverb.com API. It wraps Reverb's REST API endpoints, authenticating via a user-supplied API key.

> [!IMPORTANT]
> **API Key Authentication**: This project authenticates with the Reverb API using a personal API key passed via the `REVERB_API_KEY` environment variable or a config file. There is no OAuth flow. Do NOT add OAuth or credential encryption infrastructure.

> [!NOTE]
> **Package Manager**: Use `pnpm` instead of `npm` for Node.js package management in this repository.

## Build & Test

> [!IMPORTANT]
> **Test Coverage**: The `codecov/patch` check requires that new or modified lines are covered by tests. When adding code, extract testable helper functions rather than embedding logic in `main`/`run` where it's hard to unit-test. Run `cargo test` locally and verify new branches are exercised.

```bash
cargo build          # Build in dev mode
cargo clippy -- -D warnings  # Lint check
cargo test           # Run tests
```

## Changesets

Every PR must include a changeset file. Create one at `.changeset/<descriptive-name>.md`:

```markdown
---
"@reverbdotcom/cli": patch
---

Brief description of the change
```

Use `patch` for fixes/chores, `minor` for new features, `major` for breaking changes. The CI policy check will fail without a changeset.

## Architecture

The CLI uses a **schema-driven command generation** strategy:

1. Parse argv to extract the resource name (e.g., `listings`)
2. Look up the resource's schema definition, build a dynamic `clap::Command` tree, then re-parse

### Workspace Layout

The repository is a Cargo workspace with two crates:

| Crate | Package | Purpose |
| --- | --- | --- |
| `crates/reverb/` | `reverb` | Publishable library — core types and helpers |
| `crates/reverb-cli/` | `reverb-cli` | Binary crate — the `revcli` CLI |

#### Library (`crates/reverb/src/`)

| File | Purpose |
| --- | --- |
| `schema.rs` | Serde models for API schema + endpoint definitions |
| `services.rs` | Resource alias → endpoint/version mapping |
| `error.rs` | `RevError` enum, exit codes, JSON serialization |
| `validate.rs` | Path/URL/resource validators, `encode_path_segment()` |
| `client.rs` | HTTP client with retry logic and API key injection |

#### CLI (`crates/reverb-cli/src/`)

| File | Purpose |
| --- | --- |
| `main.rs` | Entrypoint, two-phase CLI parsing, method resolution |
| `auth.rs` | API key resolution from env vars and config file |
| `auth_commands.rs` | `revcli auth` subcommands: `set-key`, `status` |
| `commands.rs` | Recursive `clap::Command` builder from schema resources |
| `executor.rs` | HTTP request construction, response handling, schema validation |
| `schema_cmd.rs` | `revcli schema` command — introspect API endpoint schemas |
| `logging.rs` | Opt-in structured logging (stderr + file) via `tracing` |

## Input Validation & URL Safety

> [!IMPORTANT]
> This CLI may be invoked by AI/LLM agents. Always assume inputs can be adversarial — validate paths against traversal (`../../.ssh`), restrict format strings to allowlists, reject control characters, and encode user values before embedding them in URLs.

> [!NOTE]
> **Environment variables are trusted inputs.** Validation rules apply to **CLI arguments** that may be passed by untrusted agents. Environment variables (e.g. `REVERB_CLI_CONFIG_DIR`) are set by the user and are not subject to path traversal validation.

### Path Safety (`crates/reverb/src/validate.rs`)

When adding new helpers or CLI flags that accept file paths, **always validate** using the shared helpers:

| Scenario | Validator | Rejects |
| --- | --- | --- |
| File path for writing (`--output-dir`) | `validate::validate_safe_output_dir()` | Absolute paths, `../` traversal, symlinks outside CWD, control chars |
| File path for reading (`--dir`) | `validate::validate_safe_dir_path()` | Absolute paths, `../` traversal, symlinks outside CWD, control chars |
| Enum/allowlist values | clap `value_parser` | Any value not in the allowlist |

### URL Encoding (`crates/reverb-cli/src/helpers/mod.rs`)

User-supplied values embedded in URL **path segments** must be percent-encoded:

```rust
// CORRECT
let url = format!(
    "https://api.reverb.com/api/listings/{}",
    crate::helpers::encode_path_segment(listing_id),
);

// WRONG
let url = format!("https://api.reverb.com/api/listings/{}", listing_id);
```

For **query parameters**, use reqwest's `.query()` builder:

```rust
// CORRECT
client.get(url).query(&[("query", user_query)]).send().await?;

// WRONG
let url = format!("{}?query={}", base_url, user_query);
```

### Resource Name Validation

When a user-supplied string is used as a resource identifier embedded in a URL path, validate it first:

```rust
let id = crate::validate::validate_resource_name(&listing_id)?;
let url = format!("https://api.reverb.com/api/listings/{}", id);
```

### Checklist for New Features

When adding a new helper or CLI command:

1. **File paths** → Use `validate_safe_output_dir` / `validate_safe_dir_path`
2. **Enum flags** → Constrain via clap `value_parser`
3. **URL path segments** → Use `encode_path_segment()`
4. **Query parameters** → Use reqwest `.query()` builder
5. **Resource names** (listing IDs, shop slugs, etc.) → Use `validate_resource_name()`
6. **Write tests** for both the happy path AND the rejection path

## PR Labels

- `area: schema` — Schema definitions, endpoint mapping
- `area: http` — Request execution, URL building, response handling
- `area: docs` — README, contributing guides, documentation
- `area: distribution` — GitHub Actions release workflow, install methods
- `area: auth` — API key resolution, config file
- `area: helpers` — Custom helper commands

## Helper Commands (`+verb`)

Helpers are handwritten commands prefixed with `+` that provide value the schema-driven commands cannot: multi-step orchestration, format translation, or multi-endpoint composition.

> [!IMPORTANT]
> **Do NOT add a helper that** wraps a single API call already available via schema-driven commands, adds flags to expose data already in the response, or re-implements schema parameters as custom flags. Helper flags must control orchestration logic — use `--params` and `--format`/`jq` for API parameters and output filtering.

## Environment Variables

### Authentication

| Variable | Description |
|---|---|
| `REVERB_API_KEY` | Reverb personal API key (required; obtain from reverb.com/my/api_access) |

### Configuration

| Variable | Description |
|---|---|
| `REVERB_CLI_CONFIG_DIR` | Override the config directory (default: `~/.config/revcli`) |

### Logging

| Variable | Description |
|---|---|
| `REVERB_CLI_LOG` | Log level filter for stderr output (e.g., `revcli=debug`). Off by default. |
| `REVERB_CLI_LOG_FILE` | Directory for JSON-line log files with daily rotation. Off by default. |

All variables can also live in a `.env` file (loaded via `dotenvy`).
