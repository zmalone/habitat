extern crate clap;
extern crate env_logger;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_pkg_export_tar as export_tar;

use common::ui::UI;
use hcore::PROGRAM_NAME;
use export_tar::{Cli};

fn main() {
    env_logger::init().unwrap();
    let mut ui = UI::default_with_env();

    start(&mut ui)
}

fn start(ui: &mut UI) {
    let cli = cli();
}

fn cli() {
    let name: &str = &*PROGRAM_NAME;
    let about = "Creates a tar package from a Habitat package";


}
