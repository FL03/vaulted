/*
    Appellation: cmds <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::args::CRUDArgs;
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    App {
        #[clap(long, short, value_parser)]
        mode: Option<String>,
    },
    Password {
        #[clap(value_enum)]
        action: CRUDArgs,

        #[clap(long, short, value_parser)]
        length: Option<usize>,
    },
    Vault {
        #[clap(value_enum)]
        action: CRUDArgs,
        #[clap(long, short, value_parser)]
        identifier: String,
    },
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        match self {
            Self::App { mode } => {}
            Self::Password { action, length } => {
                // let length = length.unwrap_or_default();
                let action = match action.clone() {
                    CRUDArgs::Create => {
                        let length = match length {
                            Some(v) => v.clone(),
                            None => 12,
                        };
                        let password = vaulted::passwords::Password::generate(length);
                        tracing::info!("Generating a new password...");
                        println!("{}", password)
                    }
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
            Self::Vault { action, identifier } => {
                let action = match action.clone() {
                    CRUDArgs::Create => {}
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
        };
        Ok(self)
    }
    pub fn handle_crud<S, T>(&self, transition: &dyn Fn(S) -> T) -> &Self {
        self
    }
}

pub fn transitioner<S, T>(ctx: S, actor: &dyn Fn(S) -> T) -> T {
    actor(ctx)
}

pub fn app_mode_handler(mode: String) -> BoxResult {
    println!("{:?}", mode);

    Ok(())
}
