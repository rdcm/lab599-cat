 > **WIP** — work in progress, API is unstable

# lab599-cat

Rust implementation of Lab599 CAT protocol.

## Features

- Pure Rust (no C/C++)
- Typed command/response API
- Extensible device abstraction
- Suitable for TUI/CLI apps

## Example

```rust
use lab599_cat_device::Tx500;

let mut device = Tx500::new(serial_port);

device.set_frequency(7_000_000)?;
let freq = device.get_frequency()?;