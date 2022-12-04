/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::args::CRUDArgs;
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use vaulted::passwords::PasswordBuilder;

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
            Self::App { mode } => {
                tracing::info!("{:?}", mode.clone().unwrap_or_default());
            }
            Self::Password { action, length } => {
                // let length = length.unwrap_or_default();
                match *action {
                    CRUDArgs::Create => {
                        let length = match length {
                            Some(v) => *v,
                            None => 12,
                        };
                        let mut builder = PasswordBuilder::new();
                        tracing::info!("Process: Generating a new password...");
                        builder.generate(length);
                        tracing::info!("Success: Generated a new password...");
                        let password = builder.password().clone();
                        println!("{}", password)
                    }
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
            Self::Vault { action, identifier } => {
                let _id = identifier.clone();
                match *action {
                    CRUDArgs::Create => {}
                    CRUDArgs::Read => {}
                    CRUDArgs::Update => {}
                    CRUDArgs::Delete => {}
                };
            }
        };
        Ok(self)
    }
}

pub async fn handle_crud<S, T>(
    action: &CRUDArgs,
    _transition: &dyn Fn(S) -> BoxResult<T>,
) -> BoxResult {
    match *action {
        CRUDArgs::Create => {}
        CRUDArgs::Read => {}
        CRUDArgs::Update => {}
        CRUDArgs::Delete => {}
    };
    Ok(())
}

pub fn transitioner<S, T>(ctx: S, actor: &dyn Fn(S) -> T) -> T {
    actor(ctx)
}

pub fn app_mode_handler(mode: String) -> BoxResult {
    println!("{:?}", mode);

    Ok(())
}
