# loglet

A minimal, zero-heap Rust logging library for CLI applications.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
loglet = "0.0.2"
```

Or via the command line:

```bash
cargo add loglet
```

## Features

- Zero heap allocations (uses `fmt::Arguments` throughout)
- Coloured output via `owo-colors`
- Tagged log entries
- Custom log levels
- Debug logs compiled away in release builds
- Writes errors to `stderr`, everything else to `stdout`

## Usage

From [`examples/basic_usage.rs`](examples/basic_usage.rs):

```rust
use loglet::{info, warn, error, debug};

fn main() {
    info!("Starting application.");

    let user = "Alice";
    let attempts = 3;
    warn!("Failed login by '{}' (attempt #{})", user, attempts);

    error!("Database connection timed out!");

    debug!("Hidden in release builds.");

    // Tagged
    info!(tag: "auth", "User '{}' logged in.", user);
    warn!(tag: "db", "Connection pool running low.");
    error!(tag: "network", "Host unreachable after {} retries.", 5);
}
```

Check out the [`examples/`](examples/) folder for more.

## Default Log Levels

| Macro    | Prefix | Stream | Color |
|----------|--------|--------|--------|
| `info!`  | `I`    | stdout | Green  |
| `warn!`  | `W`    | stdout | Yellow |
| `error!` | `E`    | stderr | Red    |
| `debug!` | `D`    | stdout | Cyan   |

Custom levels are also supported (see [`examples/custom_level.rs`](examples/custom_level.rs))

## Examples

Clone the repo first:
 
```bash
git clone https://github.com/AltIsBacc/loglet
cd loglet
```
 
Then run any example:

```bash
cargo run --example basic_usage
cargo run --example custom_level
```

## License

MIT
