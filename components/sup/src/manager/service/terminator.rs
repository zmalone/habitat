use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use sys::service::{Process, ShutdownMethod};

static LOGKEY: &'static str = "ST"; // "Service Terminator"

pub fn terminate_service<P>(pid: u32, service_group: String, pidfile: P)
where
    P: Into<PathBuf>,
{
    let pidfile = pidfile.into();
    let _ = thread::Builder::new()
        .name(format!("terminate-{}", pid))
        .spawn(move || {
            match shutdown(pid) {
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

// TODO (CM) wanted to use Pid, but it could be negative... that was constrained
// in the Launcher implementation, based on how it was instantiated
fn shutdown(pid: u32) -> ShutdownMethod {
    debug!("Terminating: {}", pid);
    Process::new(pid).kill()
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
