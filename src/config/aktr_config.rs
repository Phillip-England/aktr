use std::fs;
use std::{io::ErrorKind, path::Path};
use std::io::Error;

use serde_derive::{Deserialize, Serialize};

pub const DEFAULT_AKTR_CONFIG_PATH: &str = ".aktr.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfigEntryPoint {
    pub root: String
}

impl AktrConfigEntryPoint {

    pub fn default() -> Self {
        AktrConfigEntryPoint {
            root: String::from("./index.tsx")
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfigSrc {
    pub dir: String,
    pub lib: String,
    pub client: String,
    pub server: String,
    pub shared: String,
    pub package_json: String,
    pub config: String,
}

impl AktrConfigSrc {

    pub fn default() -> Self {
        AktrConfigSrc {
            dir: String::from("aktr/"),
            lib: String::from("aktr/lib/"),
            client: String::from("aktr/client/"),
            server: String::from("aktr/server/"),
            shared: String::from("aktr/shared/"),
            package_json: String::from("package.json"),
            config: String::from(".aktr.toml"),
        }
    }

}



#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfig {
    pub entrypoint: AktrConfigEntryPoint,
    pub src: AktrConfigSrc
}

impl AktrConfig {

    pub fn default() -> Self {
        AktrConfig {
            entrypoint: AktrConfigEntryPoint::default(),
            src: AktrConfigSrc::default(),
        }
    }

    pub fn from_file() -> Result<Self, Error> {
        let path = Path::new("./.aktr.toml");
        let config_contents = fs::read_to_string(path);
        if config_contents.is_err() {
            return Err(Error::new(ErrorKind::NotFound, ".aktr.toml not found"));
        }
        let config = toml::from_str(config_contents.unwrap().as_str());
        if config.is_err() {
            return Err(Error::new(ErrorKind::Other, "failed to parse .aktr.toml"));
        }
        Ok(config.unwrap())
    }

    pub fn init_config_file(&self, init_destination: String) -> Result<&Self, Error> {
		if Path::new(&init_destination).exists() {
			return Err(Error::new(ErrorKind::AlreadyExists, ".aktr.toml already exists"));
		}
		if let Some(parent) = Path::new(&init_destination).parent() {
			fs::create_dir_all(parent)?;
		}
		let toml_string = toml::to_string(&self);
		if toml_string.is_err() {
			return Err(Error::new(ErrorKind::Other, "failed to serialize config to toml"));
		}
		let file = fs::write(init_destination, toml_string.unwrap());
		if file.is_err() {
			return Err(Error::new(ErrorKind::Other, "failed to write .aktr.toml"));
		}
		Ok(self)
    }

}