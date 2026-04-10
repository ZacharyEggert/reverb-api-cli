# Architecture: API CLI

A generalized architecture guide for building schema-driven, agent-friendly CLI tools that wrap external REST APIs. Based on the patterns found in `gws-cli`.

---

## Project Structure

```
<project>/
├── crates/
│   ├── <api-name>/                 # Library crate (reusable core, no CLI deps)
│   │   └── src/
│   │       ├── lib.rs              # Public API surface
│   │       ├── client.rs           # HTTP client with retry/pooling
│   │       ├── schema.rs           # API schema types (OpenAPI/Discovery/etc.)
│   │       ├── error.rs            # Core error enum
│   │       ├── services.rs         # Service registry (name → endpoint/version)
│   │       └── validate.rs         # Input validation utilities
│   │
│   └── <api-name>-cli/             # Binary crate (CLI, TUI, formatting)
│       └── src/
│           ├── main.rs             # Entry point, two-phase dispatch
│           ├── commands.rs         # Dynamic command tree builder
│           ├── executor.rs         # Request construction & execution
│           ├── auth.rs             # Auth flow orchestration
│           ├── auth_commands.rs    # CLI commands for auth (setup, login, etc.)
│           ├── credential_store.rs # Encrypted token storage
│           ├── formatter.rs        # Output formatting (JSON/Table/CSV/YAML)
│           ├── error.rs            # CLI-specific error display
│           ├── schema_cmd.rs       # Schema introspection command
│           ├── logging.rs          # Tracing setup
│           └── helpers/            # Service-specific extensions
│               ├── mod.rs          # Helper trait + registry
│               └── <service>/      # One module per service that needs custom logic
│
├── Cargo.toml                      # Workspace definition
└── Cargo.lock
```

### Why two crates?

| Concern | Library crate | Binary crate |
|---|---|---|
| API types, HTTP client | Yes | No |
| Error types, validation | Yes | No |
| clap / TUI / formatting | No | Yes |
| Auth flows, helpers | No | Yes |

The library crate can be published independently (e.g., to crates.io) and reused in other projects (bots, SDKs, test harnesses) without pulling in CLI dependencies.

---

## Core Architectural Patterns

### 1. Two-Phase CLI Parsing

The key technique that enables dynamic command trees.

**Phase 1** — Identify the resource/service from the first positional argument. Fetch its schema (from a remote schema registry, local file, or embedded constant).

**Phase 2** — Build the full `clap::Command` tree from the schema. Re-parse the original argv against that tree.

```
argv
  │
  ├─ Phase 1: extract argv[1] ("drive", "users", etc.)
  │           fetch + parse API schema for that service
  │
  └─ Phase 2: build clap tree from schema
              clap::Command::try_get_matches_from(argv)
              execute matched method
```

**Why it works:** `clap` allows constructing the command tree at runtime. The CLI never needs recompilation when the upstream API adds endpoints.

**Trade-off:** ~50–200ms startup overhead for schema fetch (mitigated by local caching).

---

### 2. Schema-Driven Command Generation

API endpoints map directly to CLI subcommands. No hardcoding.

```
Schema resource: "files"
  Method: "list"   → subcommand: <cli> files list
  Method: "create" → subcommand: <cli> files create
  Method: "delete" → subcommand: <cli> files delete
```

Each method's parameters become CLI flags:
- URL path parameters → required `--params` keys
- Query parameters → `--params` JSON object
- Request body → `--json` flag (raw JSON or file path)
- File upload methods → `--upload` flag

Standard flags added to every method:
- `--format` — output format (json | table | yaml | csv)
- `--page-all` / `--page-limit N` / `--page-delay MS` — pagination
- `--dry-run` — validate inputs without sending the request

**Schema sources to consider:**
- OpenAPI / Swagger documents
- GraphQL introspection
- API Discovery documents (Google style)
- Protocol Buffers / gRPC reflection
- Hand-authored JSON/TOML schema files

---

### 3. Trait-Based Service Extensions (`Helper` Pattern)

Most API methods are handled generically by `executor.rs`. For methods that need custom logic (e.g., a "send email" command that constructs MIME internally), use a trait:

```rust
pub trait Helper: Send + Sync {
    /// Add custom subcommands to the clap tree for this service.
    fn inject_commands(&self, cmd: Command, schema: &ApiSchema) -> Command;

    /// Handle a matched command. Return Ok(true) if handled, Ok(false) to fall through.
    fn handle<'a>(
        &'a self,
        schema: &'a ApiSchema,
        matches: &'a ArgMatches,
    ) -> Pin<Box<dyn Future<Output = Result<bool, CliError>> + Send + 'a>>;
}

pub fn get_helper(service: &str) -> Option<Box<dyn Helper>> {
    match service {
        "email"   => Some(Box::new(email::EmailHelper)),
        "storage" => Some(Box::new(storage::StorageHelper)),
        _         => None,
    }
}
```

Each helper lives in its own module under `src/helpers/`. Core code never imports service-specific modules directly.

Custom commands added by helpers can be named with a sigil (e.g., `+send`, `+upload`) to visually distinguish them from schema-generated commands in `--help` output.

---

### 4. Request Execution Layer (`executor.rs`)

Central orchestrator for all HTTP requests. Responsibilities:

1. **URL building** — substitute path parameters, append query parameters
2. **Body construction** — serialize `--json` flag value; build multipart for `--upload`
3. **Validation** — check required params, type-check values against schema
4. **Execution** — call HTTP client; handle 4xx/5xx → structured errors
5. **Pagination loop** — follow `nextPageToken` / `Link` headers / cursor fields
6. **Response handling** — extract data array, format output, write to stdout

Keep this layer API-agnostic. Pass in schema metadata, not service-specific knowledge.

---

### 5. Layered Error Handling with Structured Exit Codes

Define a small, stable enum of error categories. Each maps to a distinct exit code for scripting.

```rust
pub enum CliError {
    Api         { code: u16, message: String },  // exit 1 — upstream API error
    Auth        (String),                         // exit 2 — authentication failure
    Validation  (String),                         // exit 3 — bad user input
    Schema      (String),                         // exit 4 — schema fetch/parse failure
    Other       (anyhow::Error),                  // exit 5 — unexpected error
}
```

Rules:
- Always serialize errors as JSON to stdout (not just human-readable stderr messages) so agents and scripts can parse them.
- Print human-readable colored output to stderr separately.
- Exit codes must be stable across versions — treat them as a public API.

---

### 6. Authentication Layer

Support multiple credential sources resolved in priority order:

1. Environment variable token (`API_TOKEN` env var) — for CI, scripts
2. Environment variable credentials file path — for container deployments
3. Encrypted credentials from `~/.config/<cli>/` — for interactive users
4. Application/platform default credentials fallback — for cloud VMs

**Token storage:**
- Encrypt at rest with AES-256-GCM
- Store the encryption key in the OS keyring (Keychain on macOS, libsecret on Linux, Credential Manager on Windows)
- Fallback: store key in a separate file if no keyring available

**Token refresh:**
- Access tokens expire. Store refresh tokens and exchange them transparently.
- Use an `AccessTokenProvider` trait so long-running helpers can request fresh tokens without knowing credential internals.

---

### 7. Output Formatting Layer (`formatter.rs`)

Support at minimum: JSON (default), Table, YAML, CSV.

**Pagination-aware formatting:**
- JSON: first page → pretty-printed object; subsequent pages → NDJSON (one compact object per line)
- Table/CSV: emit headers only on first page; data-only rows on continuation
- YAML: emit `---` document separator between pages

**Smart extraction:**
- API responses often nest the data array: `{ "files": [...], "nextPageToken": "..." }`
- Detect and unwrap the data array automatically so `--format table` shows rows, not a single-column response wrapper.

**Nested field flattening for tables:**
- `{ "owner": { "name": "Alice" } }` → column `owner.name`
- Truncate deeply nested objects to keep tables readable.

---

### 8. Input Validation

**At parse boundaries (before any untrusted input is processed):**
- Strip or reject Unicode control characters (U+0000–U+001F, U+007F–U+009F)
- Reject bidirectional override characters (U+202A–U+202E, U+2066–U+2069, U+200F, U+200E)
- Reject zero-width characters (U+200B, U+FEFF, etc.)

This is especially important when the CLI is invoked by an LLM agent, since prompts embedded in API responses could otherwise inject instructions into subsequent processing.

**Path validation (for `--upload`, `--output` flags):**
- Reject paths containing `..` components
- Canonicalize and verify the path stays within the allowed directory
- Resolve symlinks before checking containment

---

### 9. Service Registry

A static table mapping short names → API base URL, version, description.

```rust
pub struct ServiceEntry {
    pub name: &'static str,          // "drive"
    pub api_name: &'static str,      // "drive"
    pub version: &'static str,       // "v3"
    pub description: &'static str,   // "Google Drive API"
}
```

Enables:
- Tab completion for service names
- `<cli> --list-services` introspection command
- Aliasing: `<cli> storage` → `<cli> storage_v2`

---

### 10. Schema Introspection Command

Expose a `schema` subcommand for users and agents to explore the API without making real requests.

```
<cli> schema <Service>.<Resource>.<Method>   # show method signature
<cli> schema <Service>.<TypeName>            # show type definition
<cli> schema <Service>.<TypeName> --resolve  # inline all $ref references
```

This makes the CLI self-documenting and removes the need for users to read API reference docs for common tasks.

---

### 11. HTTP Client (`client.rs`)

- Use a single shared client instance (via `OnceLock` or `LazyLock`) for connection pooling.
- Implement retry with exponential backoff for:
  - 429 Too Many Requests (respect `Retry-After` header)
  - 503 Service Unavailable
  - Transient network errors (connection reset, timeout)
- Cap total retry time (e.g., 60 seconds).
- Set a meaningful `User-Agent` header identifying the CLI and its version.

---

### 12. Pagination

Auto-pagination driven by cursor/token fields in the response.

```
--page-all              loop until no more pages
--page-limit N          stop after N pages (default: 10 — prevents runaway)
--page-delay MS         wait between requests (default: 100ms — be polite)
```

The cursor field name varies by API (`nextPageToken`, `next_cursor`, `Link` header). Detect the active pagination mechanism from the schema or by inspecting the first response.

---

### 13. Media Upload

For APIs that accept file uploads:

1. Detect MIME type from file extension (via `mime_guess` or equivalent).
2. Allow override with `--upload-content-type`.
3. Construct a multipart/related body:
   - Part 1: `application/json` (resource metadata from `--json`)
   - Part 2: `<mime-type>` (file content)
4. Set `uploadType=multipart` or equivalent query parameter.

---

### 14. Logging & Observability

Two output channels:

1. **Structured stderr logging** (enabled via env var, e.g., `<CLI>_LOG=debug`):
   - Use `tracing` + `tracing-subscriber`
   - Format: human-readable text in development, JSON in production/CI

2. **File logging** (enabled via `<CLI>_LOG_FILE=/path/to/dir`):
   - Daily-rotated JSON files
   - Useful for audit trails and debugging in CI

Never mix log output with stdout. stdout is reserved for structured data output that scripts and agents consume.

---

## Key Dependencies (Rust)

| Purpose | Crate |
|---|---|
| CLI parsing | `clap 4` |
| Async runtime | `tokio 1` |
| HTTP client | `reqwest 0.12` |
| Serialization | `serde`, `serde_json` |
| Error handling | `thiserror`, `anyhow` |
| Encryption | `aes-gcm` |
| OS keyring | `keyring` |
| MIME detection | `mime_guess` |
| Logging | `tracing`, `tracing-subscriber` |
| TUI (optional) | `ratatui`, `crossterm` |
| Memory safety | `zeroize` (for secrets) |

---

## Adapting This Architecture

To apply this to a different API:

| Step | What to change |
|---|---|
| Schema source | Replace Discovery/OpenAPI parsing in `schema.rs` with your format |
| Service registry | Populate `services.rs` with your API's resources |
| Auth layer | Swap OAuth for API key, JWT, HMAC, etc. — keep the `AccessTokenProvider` trait |
| URL building | Adapt `executor.rs` URL construction to your API's conventions |
| Pagination | Detect your API's cursor mechanism; plug into the existing pagination loop |
| Helpers | Add service-specific helpers for anything the schema can't express |
| Validators | Keep character/path validation as-is; add domain-specific rules |
| Formatter | Keep as-is — it's generic JSON/table/YAML/CSV |
| Error codes | Keep the exit code contract; adjust error variant names |

What you do **not** need to change: the two-phase parsing pattern, the `Helper` trait pattern, the error enum structure, the output formatter, the pagination loop design, or the logging setup. These are API-agnostic.

---

## Design Principles

1. **Schema first** — commands come from upstream metadata, not hardcoded strings.
2. **Library / binary separation** — core logic is reusable without CLI overhead.
3. **Trait-based extensibility** — add service logic without touching core files.
4. **Agent-friendly output** — structured JSON, stable exit codes, no interactive prompts by default.
5. **Defense in depth** — validate and sanitize at every trust boundary.
6. **Observable by default** — structured errors on stdout, logs on stderr, nothing silent.
7. **Polite by default** — retry with backoff, pagination delays, respect rate limits.
