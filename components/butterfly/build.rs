extern crate pkg_config;
extern crate prost;
extern crate prost_build;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    generate_protocols();
}

fn generate_protocols() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config
        .compile_protos(&protocol_files(), &protocol_includes())
        .expect("Error compiling protobuf definitions");
}

fn protocol_includes() -> Vec<String> {
    vec!["protocols".to_string()]
}

fn protocol_files() -> Vec<String> {
    let mut files = vec![];
    for entry in fs::read_dir("protocols").unwrap() {
        let file = entry.unwrap();
        // skip vim temp files
        if file.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }
        if file.metadata().unwrap().is_file() {
            files.push(file.path().to_string_lossy().into_owned());
        }
    }
    files
}
