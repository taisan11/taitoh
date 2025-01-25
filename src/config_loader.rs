use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_port")]
    pub addr: String
}

fn default_port() -> String {
    "127.0.0.1:8888".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config(file_path: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    CONFIG.set(config)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Config already set"))?;
    Ok(())
}

pub fn get_config() -> Option<&'static Config> {
    CONFIG.get()
}