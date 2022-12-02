# vaulted

[![Clippy](https://github.com/FL03/vaulted/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/vaulted/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/vaulted/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/vaulted/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/vaulted/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/vaulted/actions/workflows/rust.yml)

## About

Vaulted is a secure credential management utility, designed for complete integration with the Scattered-Systems ecosystem. The SDK implements the critical capabilities and structures required for interacting with generated artifacts, credentials, or otherwise. Leveraging the included cli, users can quickly setup a new vault and begin using is as a more complete and controlled method of managing the overwhelming number of credentials today's internet necessitates.

## Installation

Use Rust's built-in package manager, cargo, and visit [vaulted-cli](https://crates.io/crates/vaulted-cli) for more informatin on *vaulted*.

```bash
cargo install vaulted-cli

```

## Usage

- [crates.io](https://crates.io/crates/scsys)
- [docs.rs](https://docs.rs/scsys)

```rust
use vaulted::passwords::Password;

fn main() {
    let password = vaulted::passwords::Password::generate(12);
    println!("Generated Password: {:?}", password);
}
```

### Cargo Commands

#### *Build the workspace for release*

```bash
cargo build --release --workspace
```

#### *Run the help command to find out more information about availible commands*

```bash
cargo run -- -h
```

or

```bash
vaulted -h
```

### Docker

#### *Build the image locally*

```bash
docker buildx build --tag jo3mccain/vaulted:latest .
```

#### *Pull the image*

```bash
docker pull jo3mccain/vaulted:latest
docker run jo3mccain/vaulted:latest
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
