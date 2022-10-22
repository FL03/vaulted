/*
    Appellation: settings <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::{prelude::{config::{Config, Environment}, Logger, Server}, ConfigResult, collect_config_files};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub logger: Option<Logger>,
    pub server: Option<Server>,
    pub workdir: String,
}

impl Settings {
    pub fn new(logger: Option<Logger>, server: Option<Server>, workdir: String) -> Self {
        Self { logger, server, workdir }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/default.config.*", true));
        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder = builder.add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let logger = Some(Logger::from("info"));
        let server = Some(Server::default());
        let curdir = std::env::current_dir().unwrap();
        let workdir = curdir.to_str().expect("").to_string();
        Self { logger, server, workdir }
    }
}
