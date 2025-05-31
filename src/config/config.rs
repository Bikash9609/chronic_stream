use super::config_types::CStreamConfig;
use std::fs;

pub fn parse_config(path: &str) {
    let raw_config = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("FATAL: Could not read config at '{}': {}", path, err);
    });

    let parsed_config: CStreamConfig = serde_yml::from_str(&raw_config).unwrap_or_else(|err| {
        panic!("FATAL: Invalid YAML in config: {}", err);
    });

    println!("{:?}", &parsed_config);
}
