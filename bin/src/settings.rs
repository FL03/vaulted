/*
    Appellation: settings <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::{prelude::{config::{Config, Environment}, Logger, Server}, ConfigResult, try_collect_config_files};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub logger: Option<Logger>,
    pub server: Server,
    pub workdir: String,
}

impl Settings {
    pub fn new(logger: Option<Logger>, server: Server, workdir: String) -> Self {
        Self { logger, server, workdir }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8000)?
            .set_default("workdir", "/")?;

        match try_collect_config_files("**/Vaulted.*", false) {
            Err(_) => {},
            Ok(v) => { builder = builder.add_source(v); }
        };

        match std::env::var("RUST_LOG") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("logger.level", Some(v))?;
            }
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("server.port", v)?;
            }
        };

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let logger = Some(Logger::from("info"));
        let server = Server::new("127.0.0.1".to_string(), 8000);
        let curdir = std::env::current_dir().unwrap();
        let workdir = curdir.to_str().expect("").to_string();
        Self { logger, server, workdir }
    }
}
