use std::{env, fs::File};

use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};



#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}


impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret: AppConfig = match (
            File::open("metadata.yml"),
            File::open("/etc/config/metadata.yml"),
            env::var("METADATA_CONFIG"),
        ) {
            (Ok(file),_,_) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("no config file found"),
        }?;
        Ok(ret)
    }
}