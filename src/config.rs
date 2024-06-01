use std::{error::Error, net::SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Standard,
    Safe,
    Clean,
}

impl TryFrom<&str> for Mode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Standard" => Ok(Self::Standard),
            "Safe" => Ok(Self::Safe),
            "Clean" => Ok(Self::Clean),
            _ => Err(format!("Invalid mode \"{value}\"")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SwiftConfig {
    pub mode: Mode,
    pub address: SocketAddr,
    pub tor: bool,
    // pub tor: TorConfig,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TorConfig {
//     enabled: bool,
//     address: Option<SocketAddr>,
// }

impl std::default::Default for SwiftConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Standard,
            address: "127.0.0.53:53".parse().unwrap(),
            tor: false,
        }
    }
}

pub fn get_config() -> Result<SwiftConfig, Box<dyn Error>> {
    let config: SwiftConfig = confy::load_path("config.toml")?;

    Ok(config)
}
