/*
    Appellation: vaulted <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub mod app;

#[tokio::main]
async fn main() -> scsys::core::BoxResult {
    let mut app = app::App::default();
    app.run().await.expect("Failed to run the application...");
    let data = serde_json::json!(vaulted::models::Account::from_str("https://pzzld.eth.limo", "pzzld"));
    let mut arch = archive::Archive::from(".artifacts/tmp/creds");
    arch.data = data;
    arch.setup().contents()?;
    arch.save(Some("test"))?;
    arch.contents()?;

    Ok(())
}

pub(crate) mod archive {
    use scsys::core::BoxResult;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use vaulted::{create_json_file, read_dir_or, read_files_in_dir, read_json_file};


    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub struct Archive {
        pub dir: String,
        pub data: Value,
    }

    impl Archive {
        pub fn new(dir: String) -> Self {
            let data = Value::default();

            Self { dir, data }
        }
        pub fn save(&self, save_as: Option<&str>) -> std::io::Result<&Self> {
            let fname = match save_as {
                Some(v) => v,
                None => "vault"
            };
            let path = format!("{}/{}.json", self.dir, fname);
            create_json_file(path.as_str(), self.data.clone())?;
            Ok(self)
        }
        pub fn setup(&self) -> &Self {
            let reader = read_dir_or(self.dir.as_str());
            println!("{:?}", reader);
            self
        }
        pub fn contents(&self) -> BoxResult<&Self> {
            let res = read_files_in_dir(self.dir.as_str())?;
            println!("{:?}", res);
            Ok(self)
        }
    }

    impl std::convert::From<&str> for Archive {
        fn from(dir: &str) -> Self {
            Self::new(dir.to_string())
        }
    }

    impl Default for Archive {
        fn default() -> Self {
            Self::from("/tmp/credentials")
        }
    }
    pub(crate) mod utils {
        
    }
}