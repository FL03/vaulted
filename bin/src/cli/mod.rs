/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
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
    pub command: Option<cmds::Commands>,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        Self::parse()
    }
    pub fn handler(&self) -> scsys::BoxResult<&Self> {
        let cmd = self.command.clone();
        if self.command.is_some() {
            let cmd = cmd.unwrap();
            cmd.handler()?;
        }

        Ok(self)
    }
}

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::new()
    }
}
