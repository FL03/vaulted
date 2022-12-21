/*
    Appellation: builder <vaults>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

use super::Vault;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VaultBuilder {
    context: PathBuf,
    workdir: PathBuf,
    pub vault: Vault,
}

impl VaultBuilder {
    pub fn new(context: PathBuf, workdir: PathBuf) -> Self {
        let vault = Vault::default();

        Self {
            context,
            workdir,
            vault,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let a = VaultBuilder::default();
        let b = a.clone();
        assert_eq!(a, b);
    }
}
