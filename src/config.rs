#![allow(dead_code)]
#![allow(unused_imports)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    version: u32,
    host: Vec<HostEntry>,
}

#[derive(Debug, Deserialize)]
struct HostEntry {
    host: String,
    xhr: String,
}