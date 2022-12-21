/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{state::*, variants::*};

pub(crate) mod state;

pub fn new() -> State {
    State::default()
}

pub(crate) mod variants {
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Deserialize,
        EnumString,
        EnumVariantNames,
        Eq,
        Hash,
        PartialEq,
        Serialize,
    )]
    #[strum(serialize_all = "snake_case")]
    pub enum States {
        #[default]
        Idle = 0,
        Error = 1,
        Request = 2,
        Response = 3,
    }
}
