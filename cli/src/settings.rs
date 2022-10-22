/*
    Appellation: settings <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub workdir: String,
}

impl Settings {
    pub fn new(workdir: String) -> Self {
        Self { workdir }
    }
}
