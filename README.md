# vaulted

[![Docker](https://github.com/FL03/vaulted/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/docker.yml)
[![Clippy](https://github.com/FL03/pzzld-gateway/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/pzzld-gateway/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/rust.yml)

## About

Vaulted is a secure credential management utility, designed for complete integration with the Scattered-Systems ecosystem. The sdk implements the critical capabilities and structures required for interacting with generated artifacts, credentials, or otherwise. Leveraging the included cli, users can quickly setup a new vault and begin using is as a more complete and controled method of managing the overwhelming number of credentials today's internet neccessitates.

## Installation

Use Rust's built-in package manager [crates](https://crates.io/crates/package) to install *package*.

```bash
cargo install package
```

## Usage

- [crates.io](https://crates.io/crates/scsys)
- [docs.rs](https://docs.rs/scsys)

```rust
use pzzld_gateway::gateways::{convert_credentials, simple_region, Gateway};
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

### Cargo Commands

#### *Build the workspace for release*

    cargo build --release --workspace

#### *Run the help command to find out more information about availible commands*

    cargo run --bin vaulted -- -h

### Docker

#### *Build the image locally*

    docker build -t scsys/vaulted:{custom-tag} .

#### *Pull the image*

    docker pull scsys/vaulted:latest

#### *Run the image*

    docker run scsys/vaulted:latest

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)
