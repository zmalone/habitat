use std::env;
use std::path::PathBuf;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
}

pub fn key_cache() -> PathBuf {
    root().join("fixtures")
}
