/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use clap::Parser;
use serde::{Deserialize, Serialize};

pub mod args;
pub mod cmds;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "Welcome to Conduit")]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn handler(&self) -> scsys::core::BoxResult<&Self> {
        let cmd = self.command.clone();
        
        Ok(self)
    }
}
