// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use libc::{self, pid_t};
use std::ops::Neg;
use time::{Duration, SteadyTime};

use hcore::os::process::{is_alive, signal, Signal};

// TODO (CM): This must be higher up, so windows can use it, too
#[derive(Debug)]
pub enum ShutdownMethod {
    AlreadyExited,
    GracefulTermination,
    Killed,
}

// TODO (CM): Do we need this struct any longer? Is it doing anything?

pub struct Process {
    pid: pid_t,
}

impl Process {
    // TODO (CM): originally this wasn't public.
    pub fn new(pid: u32) -> Self {
        Process { pid: pid as pid_t }
    }

    /// Attempt to gracefully terminate a proccess and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    pub fn kill(&self) -> ShutdownMethod {
        let mut pid_to_kill = self.pid;
        // check the group of the process being killed
        // if it is the root process of the process group
        // we send our signals to the entire process group
        // to prevent orphaned processes.
        let pgid = unsafe { libc::getpgid(self.pid) };
        if self.pid == pgid {
            debug!(
                "pid to kill {} is the process group root. Sending signal to process group.",
                self.pid
            );
            // sending a signal to the negative pid sends it to the
            // entire process group instead just the single pid
            pid_to_kill = self.pid.neg();
        }

        // JW TODO: Determine if the error represents a case where the process was already
        // exited before we return out and assume so.

        if signal(pid_to_kill, Signal::TERM).is_err() {
            return ShutdownMethod::AlreadyExited;
        }

        let stop_time = SteadyTime::now() + Duration::seconds(8);

        loop {
            if !is_alive(pid_to_kill) {
                return ShutdownMethod::GracefulTermination;
            }
            if SteadyTime::now() < stop_time {
                continue;
            }

            match signal(pid_to_kill, Signal::KILL) {
                Ok(_) => {
                    return ShutdownMethod::Killed;
                }
                Err(_) => {
                    // JW TODO: Determine if the error represents a case where the process was already
                    // exited before we return out and assume so.

                    return ShutdownMethod::GracefulTermination;
                }
            }

            // TODO (CM): wait a little more to see if the process has
            // been killed?
        }
    }
}
