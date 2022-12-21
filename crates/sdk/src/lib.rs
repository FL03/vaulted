/*
    Appellation: vaulted <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#[cfg(feature = "core")]
pub use vaulted_core as core;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::{self, passphrases::*, passwords::*, vaults::*};
    pub use super::*;
}
