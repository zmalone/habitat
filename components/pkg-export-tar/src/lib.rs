#[macro_use]
extern crate clap;
extern crate habitat_core as hcore;
extern crate url;
extern crate base64;

extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod cli;
mod error;

pub use cli::{Cli, PkgIdentArgOptions, RegistryType};
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
