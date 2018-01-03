#[macro_use]
extern crate clap;
extern crate habitat_core as hcore;
extern crate url;
extern crate habitat_common as common;
extern crate base64;

extern crate hab;

extern crate tempdir;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod build;
pub mod cli;
mod error;
mod fs;
pub mod rootfs;
mod util;

pub use cli::{Cli, PkgIdentArgOptions};
pub use error::{Error, Result};
use common::ui::UI;
use hcore::channel;
use hcore::url as hurl;

pub use build::BuildSpec;

/// The version of this library and program when built.
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// The Habitat Package Identifier string for a Busybox package.
const BUSYBOX_IDENT: &'static str = "core/busybox-static";
/// The Habitat Package Identifier string for SSL certificate authorities (CA) certificates package.
const CACERTS_IDENT: &'static str = "core/cacerts";



pub fn export_for_cli_matches(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    let default_channel = channel::default();
    let default_url = hurl::default_bldr_url();
 
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

