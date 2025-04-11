#![allow(dead_code)]
#![allow(unused_imports)]
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub version: u32,
    pub host: Vec<HostEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostEntry {
    pub host: String,
    pub xhr: String,
}