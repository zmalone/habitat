// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate clap;
#[macro_use]
extern crate habitat_sup as sup;
extern crate habitat_core as hcore;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::process;
use std::result;

use hcore::crypto::init as crypto_init;

use sup::error::{SupError, Error};

type Handler = fn() -> result::Result<(), sup::error::SupError>;

/// The entrypoint for the Supervisor.
///
/// * Set up the logger
/// * Pull in the arguments from the Command Line, push through clap
/// * Dispatch to a function that handles that action called
/// * Exit cleanly, or if we return an `Error`, call `exit_with(E, 1)`
#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    crypto_init();

    match sup::command::run_sup(None) {
        Ok(_) => std::process::exit(0),
        Err(ref e) => {
            match e.err {
                Error::ClapError(ref clap_err) => {
                    match clap_err.kind {
                        clap::ErrorKind::HelpDisplayed => std::process::exit(0),
                        clap::ErrorKind::VersionDisplayed => std::process::exit(0),
                        _ => exit_with(e, 1),
                    }
                } 
                _ => exit_with(e, 1),
            }
        }
    }
}

/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: &SupError, code: i32) {
    println!("{}", e.to_string());
    process::exit(code)
}
