/*
    Appellation: core <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::core::*;

mod core;

#[tokio::main]
async fn main() -> scsys::core::BoxResult {
    let mut app = Interface::new();
    app.cli().expect("Failed to run the application...");

    Ok(())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Archive {
    pub dir: String,
}

impl Archive {
    pub fn new(dir: String) -> Self {
        Self { dir }
    }
}

impl std::convert::From<&str> for Archive {
    fn from(dir: &str) -> Self {
        let dir = dir.to_string();
        Self { dir }
    }
}

impl Default for Archive {
    fn default() -> Self {
        Self::from("/tmp/credentials")
    }
}