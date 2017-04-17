use std::sync::{Once, ONCE_INIT};
use std::env;

pub fn origin_setup() {
  env::set_var("HAB_CACHE_KEY_PATH", super::path::key_cache());
}
