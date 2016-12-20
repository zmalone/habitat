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
extern crate habitat_sup as sup;

use sup::command;
use sup::error::Error;

#[test]
fn help_is_displayed() {
    match command::run_sup(Some(vec!["hab-sup", "--help"])) {
        Ok(()) => panic!("Returned ok when we should have returned a ClapErr"),
        Err(ref e) => {
            match e.err {
                Error::ClapError(ref clap_err) => {
                    match clap_err.kind {
                        clap::ErrorKind::HelpDisplayed => assert!(true),
                        clap::ErrorKind::VersionDisplayed => panic!("Returned version displayed"),
                        _ => panic!("Should have returned HelpDisplayed {:?}", e),
                    }
                } 
                _ => panic!("Returned an error {:?}", e),
            }
        }
    }
}

#[test]
fn version_is_displayed() {
    match command::run_sup(Some(vec!["hab-sup", "--version"])) {
        Ok(()) => panic!("Returned ok when we should have returned a ClapErr"),
        Err(ref e) => {
            match e.err {
                Error::ClapError(ref clap_err) => {
                    match clap_err.kind {
                        clap::ErrorKind::HelpDisplayed => panic!("Returned help displayed"),
                        clap::ErrorKind::VersionDisplayed => assert!(true),
                        _ => panic!("Should have returned VersionDisplayed {:?}", e),
                    }
                } 
                _ => panic!("Returned an error {:?}", e),
            }
        }
    }
}

#[test]
fn starts_a_service() {
    command::run_sup(Some(vec!["hab-sup", "start", "core/redis"])).expect("Supervisor starts");
}
