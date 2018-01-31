#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate errno;
extern crate libc;
#[macro_use]
extern crate log;
extern crate rand;

pub mod fs_root;
pub mod mount;
pub mod error;

pub use fs_root::{FsRoot, FsRootPolicy};
pub use error::{Error, Result};
use std::process::Command;
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    env_logger::init();
    if let Err(e) = _main() {
        eprintln!("FATAL: {}", e);
        process::exit(1);
    }
}

fn _main() -> Result<()> {
    // TED TODO: Integrate this with studio_root
    let studio_path = FsRoot::in_tmpdir(FsRootPolicy::Cleanup)?;
    mount::tmpfs("tmpfs", &studio_path, None, None, None)?;
    mount::umount(studio_path, None)?;
    Ok(())
    // let matches = app().get_matches();
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("Habitat Studio")
        .arg(
            Arg::with_name("no_src_mount")
                .help("Do not mount the source path into the Studio")
                .env("NO_SRC_PATH")
                .short("n"),
        )
        .arg(
            Arg::with_name("no_artifact_mount")
                .help("Do not mount the source artifact cache path into the Studio")
                .env("NO_ARTIFACT_PATH")
                .short("N"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Prints less output for better use in scripts")
                .env("QUIET")
                .short("q"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Prints more verbose output")
                .env("VERBOSE")
                .short("v"),
        )
        .arg(
            Arg::with_name("artifact_path")
                .help("Sets the source artifact cache path")
                .env("ARTIFACT_PATH")
                .takes_value(true)
                .short("a")
                .default_value("/hab/cache/artifacts"),
        )
        .arg(
            Arg::with_name("secret_keys")
                .help("Installs secret origin keys")
                .env("HAB_ORIGIN_KEYS")
                .takes_value(true)
                .short("k")
                .default_value("$HAB_ORIGIN"),
        )
        .arg(
            Arg::with_name("studio_root")
                .help("Sets a studio root")
                .env("HAB_STUDIO_ROOT")
                .takes_value(true)
                .short("r")
                .default_value("/hab/studios/<DIR_NAME>"),
        )
        .arg(
            Arg::with_name("source_path")
                .help("Sets the source path")
                .env("SRC_PATH")
                .takes_value(true)
                .short("s")
                .default_value("$PWD"),
        )
        .arg(
            Arg::with_name("studio_type")
                .help("Sets a Studio type when creating")
                .env("STUDIO_TYPE")
                .takes_value(true)
                .short("t")
                .default_value("default")
                .possible_values(&["default", "baseimage", "busybox", "stage1"]),
        )
        .subcommand(SubCommand::with_name("build").about("Build using a Studio"))
        .subcommand(SubCommand::with_name("enter").about("Interactively enter a Studio"))
        .subcommand(SubCommand::with_name("new").about("Creates a new Studio"))
        .subcommand(SubCommand::with_name("rm").about("Destroys a Studio"))
        .subcommand(SubCommand::with_name("run").about("Run a command in a Studio"))

    // if cfg!(target_os = "linux") {
    //     app.arg(
    //         Arg::with_name("docker")
    //             .help("Use a Docker Studio instead of a chroot Studio")
    //             .short("D"),
    //     );
    // }
    // if cfg!(target_os = "windows") {
    //     app.arg(
    //         Arg::with_name("windows")
    //             .help("Use a Windows Studio instead of a Docker Studio")
    //             .short("w"),
    //     );
    // }
}
