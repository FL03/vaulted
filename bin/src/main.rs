/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{context::*, interface::*, settings::*};

pub mod cli;
pub(crate) mod context;
pub(crate) mod settings;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> scsys::BoxResult {
    println!("{:?}",  std::env::current_dir());
    println!("{:?}",  std::env::temp_dir());
    let app = App::default();
    app.setup(None)?.run().await?;
    println!("{:?}",  std::env::current_dir());
    Ok(())
}

pub(crate) mod interface {
    use super::{cli::CommandLineInterface, Context, Settings};
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    pub struct App {
        ctx: Context,
    }

    impl App {
        pub fn new(ctx: Context) -> Self {
            Self { ctx }
        }
        pub fn settings(&self) -> &Settings {
            &self.ctx.settings
        }
        pub fn setup(&self, workdir: Option<&str>) -> BoxResult<&Self> {
            let logger = self.settings().logger.clone().unwrap_or(scsys::prelude::Logger::from("info"));
            logger.setup();

            let workdir = match workdir {
                Some(v) => std::path::PathBuf::from(v),
                None => {
                    let mut path = std::env::current_dir().unwrap_or_default();
                    path.push(".vault");
                    path
                }
            };
            std::fs::create_dir_all(workdir.clone())?; // Attempts to create the working directory at the specified root
            std::env::set_current_dir(workdir.clone())?;
            tracing::info!("{:?}", workdir);

            let mut tmp = workdir.clone();
            tmp.push("credentials");

            std::fs::create_dir_all(tmp)?; // Attempts to create the app directory within /tmp/vaulted

            Ok(self)
        }
        pub async fn run(&self) -> BoxResult<&Self> {
            let cli = CommandLineInterface::new();
            tracing::info!("{:?}", cli.handler()?);

            Ok(self)
        }
    }

    impl std::convert::From<Settings> for App {
        fn from(settings: Settings) -> Self {
            Self::new(Context::new(settings))
        }
    }
}
