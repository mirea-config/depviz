use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config<'a> {
    vizpath: &'a str,
    package: &'a str,
    maxdepth: u8,
}

pub fn load_config(file_path: &str) -> Config {
    let data = fs::read_to_string(file_path).expect(format!("failed to read xml file at '{}'", file_path).as_str());
    let cfg = match serde_xml_rs::from_str(&data) {
        Ok(c) => c,
        Err(e) => panic!("failed to parse xml: {e}")
    };

    return cfg;
}
