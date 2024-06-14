use std::fs::{self, File};
use std::{io::ErrorKind, path::Path};
use std::io::Error;

use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfigEntryPoint {
    pub root: String
}

impl AktrConfigEntryPoint {

    pub fn default() -> Self {
        AktrConfigEntryPoint {
            root: String::from("./index.ts")
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfigOutput {
    pub dir: String,
    pub lib: String,
    pub client: String,
    pub server: String,
    pub shared: String,
}

impl AktrConfigOutput {

    pub fn default() -> Self {
        AktrConfigOutput {
            dir: String::from("./aktr/"),
            lib: String::from("./aktr/lib/"),
            client: String::from("./aktr/client/"),
            server: String::from("./aktr/server/"),
            shared: String::from("./aktr/shared/"),
        }
    }

}



#[derive(Debug, Serialize, Deserialize)]
pub struct AktrConfig {
    pub entrypoint: AktrConfigEntryPoint,
    pub output: AktrConfigOutput
}

impl AktrConfig {

    pub fn default() -> Self {
        AktrConfig {
            entrypoint: AktrConfigEntryPoint::default(),
            output: AktrConfigOutput::default(),
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

    pub fn init_config_file(&self) -> Result<(), Error> {
        let file = File::open("./.aktr.toml");
        if file.is_ok() {
            return Err(Error::new(ErrorKind::AlreadyExists, ".aktr.toml already exists"));
        }
        let toml_string = toml::to_string(&self);
        if toml_string.is_err() {
            return Err(Error::new(ErrorKind::Other, "failed to serialize config to toml"));
        }
        let path = Path::new("./.aktr.toml");
        let file = fs::write(path, toml_string.unwrap());
        if file.is_err() {
            return Err(Error::new(ErrorKind::Other, "failed to write .aktr.toml"));
        }
        Ok(())
    }

}