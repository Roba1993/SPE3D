use std::convert::From;

#[derive(Default, Debug, Clone)]
pub struct Config(::spe3d::config::Config);

impl Config {
    pub fn new() -> Config {
        Config (::spe3d::config::Config::new())
    }

    pub fn get(&self) -> ::spe3d::config::ConfigData {
        self.0.get()
    }
}

/*impl From<Config> for ::rocket::Config {
    fn from(data: Config) -> Self {
        let data = data.0.get();
        let builder = ::rocket::config::ConfigBuilder::new(::rocket::config::Environment::Staging);

        builder
            .address(data.webserver_ip)
            .port(data.webserver_port as u16)
            .expect("The rocket configuration is bad!")
    }
}*/

impl From<Config> for ::spe3d::config::Config {
    fn from(data: Config) -> Self {
        data.0.clone()
    }
}
