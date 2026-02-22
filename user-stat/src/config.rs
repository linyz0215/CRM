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
    pub db_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}


impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret: AppConfig = match (
            File::open("user_stat.yml"),
            File::open("/etc/config/user_stat.yml"),
            env::var("USER_STAT_CONFIG"),
        ) {
            (Ok(file),_,_) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("no config file found"),
        }?;
        Ok(ret)
    }
}