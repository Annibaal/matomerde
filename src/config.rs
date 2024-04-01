use std::usize;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    element_position: Position,
    server: Server,
    client: Client,
    time: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    #[serde(default = "position_default")]
    ip: usize,
    #[serde(default = "position_default")]
    date: usize,
    #[serde(default = "position_default")]
    time: usize,
    #[serde(default = "position_default")]
    result: usize,
    #[serde(default = "position_default")]
    request_type: usize,
    #[serde(default = "position_default")]
    user_agent: usize,
    #[serde(default = "position_default")]
    path: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    ip: String,
    port: usize,
    private_key: String,
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Client {
    server_ip: String,
    server_url: String,
    port: usize,
    server_public_key: String,
}

impl Config {
    fn new(file_path: String) -> Self {
        let f = std::fs::File::open(file_path).expect("Could not open file.");
        let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
        return scrape_config;
    }
}

fn position_default() -> usize {
    99
}
