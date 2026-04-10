# Reverb CLI (`revcli`)

A command-line interface for the [Reverb.com](https://reverb.com) API. Manage listings, orders, conversations, and your
shop directly from the terminal.

## Installation

### Homebrew

```bash
brew install reverbdotcom/tap/revcli
```

### npm / npx

```bash
npm install -g @reverbdotcom/cli
# or run without installing
npx @reverbdotcom/cli listings list
```

### Cargo

```bash
cargo install reverb-cli
```

### GitHub Releases

Download the latest binary for your platform from the [Releases](https://github.com/reverbdotcom/cli/releases) page.

## Authentication

Get a personal API key from [reverb.com/my/api_access](https://reverb.com/my/api_access), then:

```bash
export REVERB_API_KEY=your_key_here
# or store it permanently
revcli auth set-key
```

## Usage

```bash
revcli <resource> <method> [flags]
```

### Examples

```bash
# List your listings
revcli listings list --params '{"per_page": 10}'

# Get a specific listing
revcli listings get --params '{"id": "12345678"}'

# Create a listing
revcli listings create --json '{"make": "Fender", "model": "Stratocaster"}'

# List all orders
revcli orders list --page-all

# Check schema for a resource
revcli schema listings.list
```

### Key Flags

| Flag                | Description                                   |
|---------------------|-----------------------------------------------|
| `--params '<JSON>'` | URL/query parameters                          |
| `--json '<JSON>'`   | Request body (POST/PUT/PATCH)                 |
| `--page-all`        | Auto-paginate, output NDJSON                  |
| `--page-limit N`    | Stop after N pages (default: 10)              |
| `--page-delay MS`   | Delay between pages (default: 100ms)          |
| `--format <fmt>`    | Output format: `json`, `table`, `yaml`, `csv` |
| `--dry-run`         | Validate without sending request              |

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
```

## License

MIT
