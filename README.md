# vaulted

[![Clippy](https://github.com/Scattered-Systems/vaulted/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Scattered-Systems/vaulted/actions/workflows/rust-clippy.yml)
[![Rust](https://github.com/Scattered-Systems/vaulted/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/vaulted/actions/workflows/rust.yml)

Vaulted is a secure credential management utility, designed for complete integration with the Scattered-Systems ecosystem. The sdk implements the critical capabilities and structures required for interacting with generated artifacts, credentials, or otherwise. Leveraging the included cli, users can quickly setup a new vault and begin using is as a more complete and controled method of managing the overwhelming number of credentials today's internet neccessitates.

## Getting Started

### Building from the source

    git clone https://github.com/scattered-systems/vaulted
    cd vaulted

#### _Build the workspace for release_

    cargo build --release --workspace

#### _Run the help command to find out more information about availible commands_

    cargo run --bin vaulted -- -h

### Docker

#### _Build the image locally_

    docker build -t scsys/vaulted:{custom-tag} .

#### _Pull the image_

    docker pull scsys/vaulted:latest

#### _Run the image_

    docker run scsys/vaulted:latest
