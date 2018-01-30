use error::*;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::ops::Deref;
use toml;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct Config {
    data: Arc<RwLock<ConfigData>>
}

impl Config {
    pub fn new() -> Config {
        let data = match ConfigData::from_config_file() {
            Ok(c) => c,
            Err(_) => ConfigData::default()
        };

        Config {
            data: Arc::new(RwLock::new(data))
        }
    }

    pub fn get(&self) -> ConfigData {
        match self.data.read() {
            Ok(c) => c.clone(),
            Err(_) => ConfigData::default()
        }
    }
}

impl From<Config> for ::rocket::Config {
    fn from(data: Config) -> Self {
        let data = data.get();
        let builder = ::rocket::config::ConfigBuilder::new(::rocket::config::Environment::Staging);

        builder
            .address(data.webserver_ip)
            .port(data.webserver_port as u16)
            .expect("The rocket configuration is bad!")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub webserver_ip: String,
    pub webserver_port: usize,
    pub websocket_port: usize,

    pub share_online: Option<ConfigShareOnline>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigShareOnline {
    pub username: String,
    pub password: String,
}

impl ConfigData {
    fn from_config_file() -> Result<ConfigData> {
        let mut config_text = String::new();
        let mut config_file = File::open("./config/config.toml")?;
        config_file.read_to_string(&mut config_text)?;

        Ok(toml::from_str::<ConfigData>(&config_text)?)
    }
}

impl Default for ConfigData {
    fn default() -> ConfigData { 
        ConfigData {
            webserver_ip: "0.0.0.0".to_string(),
            webserver_port: 8000,
            websocket_port: 8001,

            share_online: None
        }
    }
}

