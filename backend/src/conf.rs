use serde::Deserialize;
use std::{
    fs::File,
    io::{self, Read},
};

use crate::DATA_DIRS;

#[derive(Deserialize, Clone)]
pub struct CommonNodeDetails {
    pub rpc_username: String,
    pub rpc_password: String,
    pub port: u16,
    pub rpc_port: u16,
    pub wallet_name: String,
    #[serde(default = "empty_datadir")]
    pub data_dir: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub addnode: String,
    pub block_time: u16,
    pub mining: CommonNodeDetails,
    pub hot: CommonNodeDetails,
    pub cold: CommonNodeDetails,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let args: Vec<String> = std::env::args().collect();

        let path = args[1].as_str();

        let mut contents = String::new();
        let mut file = File::open(path)?;

        file.read_to_string(&mut contents)?;

        let mut config = toml::from_str::<Self>(&contents).unwrap();

        config.mining.data_dir = DATA_DIRS.mining.to_string_lossy().to_string();
        config.cold.data_dir = DATA_DIRS.cold.to_string_lossy().to_string();
        config.hot.data_dir = DATA_DIRS.hot.to_string_lossy().to_string();

        Ok(config)
    }
}

pub fn empty_datadir() -> String {
    String::default()
}
