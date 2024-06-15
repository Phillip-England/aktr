use std::{collections::HashMap, fs::File, io::{Error, ErrorKind, Write}};
use std::path::Path;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
    name: String,
    module: String,
    #[serde(rename = "type")]
    package_type: String,
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, String>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: HashMap<String, String>,
}

impl PackageJson {

    pub fn default() -> Self {
        let mut dev_dependencies = HashMap::new();
        dev_dependencies.insert("@types/bun".to_string(), "latest".to_string());

        let mut peer_dependencies = HashMap::new();
        peer_dependencies.insert("typescript".to_string(), "^5.0.0".to_string());

        PackageJson {
            name: "example".to_string(),
            module: "index.ts".to_string(),
            package_type: "module".to_string(),
            dev_dependencies: dev_dependencies,
            peer_dependencies: peer_dependencies,
        }
    }

    pub fn write_to_file(self: &Self, path: &str) -> Result<(), Error> {
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        let file_content = serde_json::to_string_pretty(self)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "could not serialize package.json"))?;
        file.write_all(file_content.as_bytes())?;
        Ok(())
    }

}