use std::error::Error;
use config::{Config, File};
use std::path::Path;

use std::fmt;
use std::str::FromStr;

enum Protocol {
    HTTP,
    HTTPS,
    Websocket,
    MQTT,
    Docker,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
            Protocol::Websocket => "ws",
            Protocol::MQTT => "mqtt",
            Protocol::Docker => "docker",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Protocol {
    type Err = ();

    fn from_str(input: &str) -> Result<Protocol, Self::Err> {
        match input {
            "http" => Ok(Protocol::HTTP),
            "https" => Ok(Protocol::HTTPS),
            "ws" => Ok(Protocol::Websocket),
            "mqtt" => Ok(Protocol::MQTT),
            "docker" => Ok(Protocol::Docker),
            _ => Err(()),
        }
    }
}

struct ServiceConfig {
    name: String,
    protocol: Protocol,
    port: u16,
    host: String,
    api: String,
    request_message: String,
    success_message: String,
    period: u64,
    timeout: u64,
    retries: u8,
}

enum SupportedConfigFormat {
    TOML,
    YAML,
    JSON,
}

impl SupportedConfigFormat {
    fn as_str(&self) -> &'static str {
        match self {
            SupportedConfigFormat::TOML => "toml",
            SupportedConfigFormat::YAML => "yaml",
            SupportedConfigFormat::JSON => "json",
        }
    }
}

type ConfigReadingError = ();

pub fn read_config(file: &Path) -> Result<Config, ConfigReadingError> {
    let config = Config::builder()
        .add_source(File::with_name(file.to_str().expect("Error reading the file path")))
        .build();

    config.try_into().expect("Error reading the configuration file")
}
