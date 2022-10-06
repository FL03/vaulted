/*
    Appellation: core <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{
    interface::Interface,
    primitives::{constants::*, types::*},
};

pub mod cli;
mod interface;
mod primitives;
