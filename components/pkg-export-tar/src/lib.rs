#[macro_use]
extern crate clap;
extern crate habitat_core as hcore;
extern crate url;

pub mod cli;

pub use cli::{Cli};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
