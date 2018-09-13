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
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use toml;
use std::sync::atomic::{AtomicUsize, Ordering};


static IDCOUNTER: AtomicUsize = AtomicUsize::new(1);

/// The Config element which can be easily shared between different threads and lifetimes.
#[derive(Default, Debug, Clone)]
pub struct Config {
    data: Arc<RwLock<ConfigData>>,
}

impl Config {
    pub fn new() -> Config {
        let data = match ConfigData::from_config_file() {
            Ok(c) => c,
            Err(_) => ConfigData::default(),
        };

        Config {
            data: Arc::new(RwLock::new(data)),
        }
    }

    pub fn get(&self) -> ConfigData {
        match self.data.read() {
            Ok(c) => c.clone(),
            Err(_) => ConfigData::default(),
        }
    }

    /// Returns a copy of the server config
    pub fn get_server(&self) -> ConfigServer {
        match self.data.read() {
            Ok(d) => d.server.clone(),
            Err(_) => ConfigServer::default(),
        }
    }

    /// Set the given server config for the global config
    pub fn set_server(&self, server: ConfigServer) -> Result<()> {
        // when settings have changed, update them and save to config file
        if server != self.get_server() {
            self.data.write()?.server = server;
            self.data.read()?.to_config_file()?;
        }

        Ok(())
    }

    /// Add a new account to the config
    pub fn add_account(&self, account: ConfigAccount) -> Result<()> {
        let mut account = account;
        account.id = IDCOUNTER.fetch_add(1, Ordering::SeqCst);
        self.data.write()?.accounts.push(account);
        self.data.read()?.to_config_file()?;
        Ok(())
    }

    /// Removes a specific account by it's id
    pub fn remove_account(&self, id: usize) -> Result<()> {
        self.data.write()?.accounts.retain(|ref a| a.id != id);
        self.data.read()?.to_config_file()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigData {
    pub server: ConfigServer,
    pub accounts: Vec<ConfigAccount>,
}

impl ConfigData {
    fn from_config_file() -> Result<ConfigData> {
        // get config from file
        let mut config_text = String::new();
        let mut config_file = File::open("./config/config.toml")?;
        config_file.read_to_string(&mut config_text)?;

        // get server config
        let tml = toml::from_str::<toml::Value>(&config_text)?;
        let server: ConfigServer = toml::from_str(&tml["server"].as_str().unwrap_or(""))
            .unwrap_or(ConfigServer::default());
        let mut accounts = vec![];

        // get account config
        if let Some(so) = tml["share_online"].as_array() {
            for s in so {
                accounts.push(ConfigAccount {
                    id: IDCOUNTER.fetch_add(1, Ordering::SeqCst),
                    hoster: ConfigHoster::ShareOnline,
                    username: s["username"].as_str().unwrap_or("").to_string(),
                    password: s["password"].as_str().unwrap_or("").to_string(),
                    status: ConfigAccountStatus::Unknown,
                });
            }
        }

        Ok(ConfigData { server, accounts })
    }

    fn to_config_file(&self) -> Result<()> {
        let mut out = String::from("# This is the SPE3D config file\n");
        out.push_str("# it gets overiten by the server from time to time!\n\n");
        out.push_str("[server]\n");
        out.push_str(&toml::to_string_pretty(&self.server)?);
        out.push_str("\n");

        for a in &self.accounts {
            out.push_str(&format!("\n[[{}]]\n", a.hoster));
            out.push_str(&format!("username = '{}'\n", a.username));
            out.push_str(&format!("password = '{}'\n", a.password));
        }

        let mut file = File::create("./config/config.toml")?;
        file.write_all(&out.into_bytes())?;

        Ok(())
    }

    pub fn get_first_so(&self) -> Option<ConfigAccount> {
        for a in &self.accounts {
            if a.hoster == ConfigHoster::ShareOnline {
                return Some(a.clone());
            }
        }

        None
    }
}

impl Default for ConfigData {
    fn default() -> ConfigData {
        ConfigData {
            server: ConfigServer::default(),
            accounts: vec![],
        }
    }
}

/// Server Configuration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigAccount {
    #[serde(skip_deserializing)]
    pub id: usize,
    pub hoster: ConfigHoster,
    pub username: String,
    pub password: String,
    pub status: ConfigAccountStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConfigAccountStatus {
    Unknown,
    NotValid,
    Free,
    Premium
}

impl Default for ConfigAccountStatus {
    fn default() -> ConfigAccountStatus {
        ConfigAccountStatus::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConfigHoster {
    ShareOnline,
    Unknown(String),
}

impl ::std::fmt::Display for ConfigHoster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ConfigHoster::ShareOnline => write!(f, "share_online"),
            ConfigHoster::Unknown(s) => write!(f, "{}", s),
        }
    }
}
