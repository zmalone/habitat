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

// NOTE: All this code is basically copied verbatim from its previous home in
// the Launcher module. Once all the service-related functionality that we're
// going to move over to the Supervisor has been moved, we can take a look at
// perhaps refactoring some of this a bit.

use kernel32;
use std::collections::HashMap;
use std::io;
use std::mem;
use time::{Duration, SteadyTime};
use winapi;

use hcore::os::process::windows_child::{ExitStatus, Handle};
use hcore::os::process::{handle_from_pid, Pid};
use sys::ShutdownMethod;

const PROCESS_ACTIVE: u32 = 259;
type ProcessTable = HashMap<winapi::DWORD, Vec<winapi::DWORD>>;

/// Kill a service process
pub fn kill(pid: Pid) -> ShutdownMethod {
    match handle_from_pid(pid) {
        None => {
            // Assume it's already gone if we can't resolve a proper process handle
            ShutdownMethod::AlreadyExited
        }
        Some(handle_ptr) => {
            let mut process = Process::new(Handle::new(handle_ptr));
            process.kill()
        }
    }
}

///////////////////////////////////////////////////////////////////////
// Private Code

struct Process {
    handle: Handle,
    last_status: Option<ExitStatus>,
}

impl Process {
    fn new(handle: Handle) -> Self {
        Process {
            handle: handle,
            last_status: None,
        }
    }

    fn id(&self) -> u32 {
        unsafe { kernel32::GetProcessId(self.handle.raw()) as u32 }
    }

    fn kill(&mut self) -> ShutdownMethod {
        if self.status().is_some() {
            return ShutdownMethod::AlreadyExited;
        }

        let ret = unsafe {
            kernel32::GenerateConsoleCtrlEvent(winapi::wincon::CTRL_BREAK_EVENT, self.id())
        };

        if ret == 0 { // 0 = error
            println!(
                "Failed to send ctrl-break to pid {}: {}",
                self.id(),
                io::Error::last_os_error()
            );
        }

        let stop_time = SteadyTime::now() + Duration::seconds(8);

        loop {
            if ret == 0 || SteadyTime::now() > stop_time {
                let proc_table = build_proc_table();
                terminate_process_descendants(&proc_table, self.id());
                return ShutdownMethod::Killed;
            }

            if self.status().is_some() {
                return ShutdownMethod::GracefulTermination;
            }
        }
    }

    fn status(&mut self) -> Option<ExitStatus> {
        if self.last_status.is_some() {
            return self.last_status;
        }
        match exit_code(&self.handle) {
            Some(PROCESS_ACTIVE) => None,
            Some(code) => {
                self.last_status = Some(ExitStatus::from(code));
                self.last_status
            }
            None => None,
        }
    }
}

fn build_proc_table() -> ProcessTable {
    let processes_snap_handle =
        unsafe { kernel32::CreateToolhelp32Snapshot(winapi::TH32CS_SNAPPROCESS, 0) };

    if processes_snap_handle == winapi::INVALID_HANDLE_VALUE {
        error!(
            "Failed to call CreateToolhelp32Snapshot: {}",
            io::Error::last_os_error()
        );
        return ProcessTable::new();
    }
    let mut table = ProcessTable::new();
    let mut process_entry = winapi::PROCESSENTRY32W {
        dwSize: mem::size_of::<winapi::PROCESSENTRY32W>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; winapi::MAX_PATH],
    };
    // Get the first process from the snapshot.
    match unsafe { kernel32::Process32FirstW(processes_snap_handle, &mut process_entry) } {
        1 => {
            // First process worked, loop to find the process with the correct name.
            let mut process_success: i32 = 1;
            // Loop through all processes until we find one where `szExeFile` == `name`.
            while process_success == 1 {
                let children = table
                    .entry(process_entry.th32ParentProcessID)
                    .or_insert(Vec::new());
                (*children).push(process_entry.th32ProcessID);
                process_success =
                    unsafe { kernel32::Process32NextW(processes_snap_handle, &mut process_entry) };
            }
            unsafe { kernel32::CloseHandle(processes_snap_handle) };
        }
        0 | _ => unsafe {
            kernel32::CloseHandle(processes_snap_handle);
        },
    }
    table
}

fn exit_code(handle: &Handle) -> Option<u32> {
    let mut exit_code: u32 = 0;
    unsafe {
        let ret = kernel32::GetExitCodeProcess(handle.raw(), &mut exit_code as winapi::LPDWORD);
        if ret == 0 {
            error!(
                "Failed to retrieve Exit Code: {}",
                io::Error::last_os_error()
            );
            return None;
        }
    }
    Some(exit_code)
}

fn terminate_process_descendants(table: &ProcessTable, pid: winapi::DWORD) {
    if let Some(children) = table.get(&pid) {
        for child in children {
            terminate_process_descendants(table, child.clone());
        }
    }
    unsafe {
        if let Some(h) = handle_from_pid(pid) {
            println!("About to terminate child process {:?}", h);
            // 1 = the exit code the terminated process will have
            if kernel32::TerminateProcess(h, 1) == 0 {
                error!(
                    "Failed to call TerminateProcess on pid {}: {}",
                    pid,
                    io::Error::last_os_error()
                );
            }
        }
    }
}
