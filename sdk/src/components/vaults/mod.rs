/*
    Appellation: vaults <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{
    access::VaultAccess,
    vault::{Vault, VaultMetadata},
};

mod access;
mod vault;
