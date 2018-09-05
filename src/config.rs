//! To start the download manager correctly a configuration is needed.
//!
//! The following shows a default configuration which allows a quick start of the component.
//! It tries to read the `config/config.toml` file and use default values if is not available.
//! ```
//! use spe3d::Config;
//!
//! let config = Config::new();
//! ```
//!
//! The config can be used through the `get()` function. It always creates a clone for you.
//! ```
//! use spe3d::Config;
//!
//! let config = Config::new();
//! let myconfig = config.get();
//! assert_eq!("0.0.0.0", myconfig.webserver_ip);
//! ```

use error::*;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};
use toml;

/// The Config element which can be easily shared between different threads and lifetimes.
#[derive(Default, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub server: ConfigServer,
    pub share_online: Vec<ConfigShareOnline>,
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
            server: ConfigServer::default(),
            share_online: vec!()
        }
    }
}

/// Server Configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigServer {
    pub ip: String,
    pub webserver_port: usize,
    pub websocket_port: usize,
}

impl Default for ConfigServer {
    fn default() -> ConfigServer { 
        ConfigServer {
            ip: "0.0.0.0".to_string(),
            webserver_port: 8000,
            websocket_port: 8001,
        }
    }
}

/// Share-Online account configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigShareOnline {
    pub username: String,
    pub password: String,
}



