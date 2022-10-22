/*
    Appellation: primitives <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{constants::*, statics::*, types::*};

pub(crate) mod constants {

    pub const TMP_DIR: &str = ".artifacts/data/tmp";
}

pub(crate) mod statics {}

pub(crate) mod types {}