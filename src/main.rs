mod config;
mod schedulers;

use config::parse_config;
use std::env;
use std::path::PathBuf;

const CSTREAM_FILE_NAME: &str = "cstream.yaml";

fn main() {
    let project_root = env::current_dir().expect("Failed to get current directory");
    let mut cstream_path = PathBuf::from(project_root);
    cstream_path.push(CSTREAM_FILE_NAME);

    let path: &str = cstream_path.to_str().unwrap();
    parse_config(path);
}
