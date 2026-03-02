use std::fs::{File};

use serde::Deserialize;
use anyhow::{Result, bail};






#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub sender_email: String,
    pub metadata: String,
    pub user_stats: String,
    pub notification: String,
}



impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret = match (
            File::open("crm.yml"),
            File::open("/etc/config/crm.yml"),
            std::env::var("CRM_CONFIG"),
        ) {
            (Ok(file),_,_) => serde_yaml::from_reader(file),
            (_, Ok(file), _) => serde_yaml::from_reader(file),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("no config file found"),
        }?;
        Ok(ret)
    }
}