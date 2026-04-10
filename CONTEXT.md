# Reverb CLI (`revcli`) Context

The `revcli` CLI provides access to the Reverb.com API (listings, orders, conversations, shop management, etc.) by
wrapping REST endpoints with a consistent command interface.

## Rules of Engagement for Agents

* **Schema Introspection:** *If you don't know the exact JSON payload structure, run `revcli schema <resource>.<method>`
  first to inspect the schema before executing.*
* **Context Window Protection:** *Reverb API responses can be large. ALWAYS use `--params '{"per_page": 5}'` or field
  filters when listing resources to avoid overwhelming your context window.*
* **Dry-Run Safety:** *Always use the `--dry-run` flag for mutating operations (create, update, delete) to validate your
  JSON payload before actual execution.*

## Authentication

`revcli` authenticates using a personal API key from Reverb. Set it once:

```bash
export REVERB_API_KEY=your_api_key_here
```

Or store it permanently:

```bash
revcli auth set-key
```

Obtain your API key at: https://reverb.com/my/api_access

## Core Syntax

```bash
revcli <resource> <method> [flags]
```

Use `--help` to get help on available commands.

```bash
revcli --help
revcli <resource> --help
revcli <resource> <method> --help
```

### Key Flags

- `--params '<JSON>'`: URL/query parameters (e.g., `per_page`, `page`, `query`).
- `--json '<JSON>'`: Request body for POST/PUT/PATCH methods.
- `--page-all`: Auto-paginates results and outputs NDJSON (one JSON object per line).
- `--output <PATH>`: Destination for binary downloads.
- `--dry-run`: Validate inputs without sending the request.
- `--format <FORMAT>`: Output format: `json` (default), `table`, `yaml`, `csv`.

## Usage Patterns

### 1. Reading Data (GET/LIST)

```bash
# List your listings
revcli listings list --params '{"per_page": 10}'

# Get a specific listing
revcli listings get --params '{"id": "12345678"}'

# Search listings
revcli listings list --params '{"query": "fender stratocaster", "per_page": 5}'
```

### 2. Writing Data (POST/PUT/PATCH)

```bash
# Create a listing
revcli listings create --json '{"make": "Fender", "model": "Stratocaster", "condition": {"uuid": "ae4d9114-1bd7-4ec5-a4a6-a94e4dad2f41"}}'

# Update a listing
revcli listings update --params '{"id": "12345678"}' --json '{"price": {"amount": "999.00", "currency": "USD"}}'
```

### 3. Pagination (NDJSON)

```bash
# Stream all of your listings
revcli listings list --page-all

# Stream all orders
revcli orders list --page-all
```

### 4. Schema Introspection

```bash
revcli schema listings.list
revcli schema listings.create
revcli schema orders.get
```
