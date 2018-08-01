use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use hcore::os::process::Pid;

use sys::service;
use sys::ShutdownMethod;
static LOGKEY: &'static str = "ST"; // "Service Terminator"

pub fn terminate_service<P>(pid: Pid, service_group: String, pidfile: P)
where
    P: Into<PathBuf>,
{
    let pidfile = pidfile.into();
    let _ = thread::Builder::new()
        .name(format!("terminate-{}", pid))
        .spawn(move || {
            debug!("Terminating: {}", pid);
            match service::kill(pid) {
                ShutdownMethod::AlreadyExited => {
                    outputln!(preamble service_group, "Already exited: {:?}", pid);
                }
                ShutdownMethod::GracefulTermination => {
                    outputln!(preamble service_group, "Gracefully terminated {:?}", pid);
                }
                ShutdownMethod::Killed => {
                    outputln!(preamble service_group, "Had to kill {:?}", pid);
                }
            }

            // We remove the pidfile here because we've either shut
            // the service down gracefully, or it's been killed.
            cleanup_pidfile(pidfile);
        });
}

// This is a dupe of Supervisor::cleanup_pidfile. That is still used
// in other shutdown scenarios... once we've consolidated everything,
// use just one thing.
//
// TODO (CM): Also suggests that I need to address restarting in this
// work, as well :/
fn cleanup_pidfile<P>(pidfile: P)
where
    P: AsRef<Path>,
{
    let pidfile = pidfile.as_ref();
    debug!("Attempting to clean up pid file {}", pidfile.display());
    match fs::remove_file(pidfile) {
        Ok(_) => debug!("Removed pid file"),
        Err(e) => debug!("Error removing pidfile: {}, continuing", e),
    }
}
