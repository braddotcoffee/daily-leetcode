use serde_yaml;
use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind},
};

pub struct Config {
    data: serde_yaml::Value,
    secrets: serde_yaml::Value,
}

impl Config {
    pub fn load() -> Config {
        let f = File::open("config.yaml").unwrap();
        let data: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
        let f = File::open("secrets.yaml").unwrap();
        let secrets: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
        Config { data, secrets }
    }

    pub fn get_discord_token(&self) -> Result<String, Box<dyn Error>> {
        self.secrets["Discord"]["Token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(Box::from(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to load Discord token",
            )))
    }

    pub fn get_channel_id(&self) -> Result<u64, Box<dyn Error>> {
        self.data["Discord"]["ChannelID"]
            .as_u64()
            .ok_or(Box::from(io::Error::new(
                ErrorKind::InvalidData,
                "Failed to load LeetCode daily channel id",
            )))
    }
}
