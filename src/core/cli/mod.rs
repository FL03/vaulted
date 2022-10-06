/*
    Appellation: mod <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{
    args::{CRUDArgs, PowerArgs},
    cmds::Commands,
};
use clap::Parser;
use serde::{Deserialize, Serialize};

mod args;
mod cmds;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Welcome to Conduit")]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

impl CommandLineInterface {
    pub fn data() -> Self {
        Self::parse()
    }
}
