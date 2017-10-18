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

pub mod hooks;
mod composite_spec;
mod config;
mod health;
mod package;
// mod updater;
mod spec;

use std;
use std::borrow::Borrow;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use ansi_term::Colour::{Yellow, Red, Green};
use butterfly::rumor::service::Service as ServiceRumor;
use hcore::crypto::hash;
use hcore::fs::FS_ROOT_PATH;
use hcore::os::process::{self, Pid};
use hcore::package::PackageInstall;
use hcore::service::ServiceGroup;
use hcore::util::perm::{set_owner, set_permissions};
use launcher_client::LauncherCli;
use time::{self, Timespec};
use zmq;

pub use self::config::Cfg;
pub use self::health::{HealthCheck, SmokeCheck};
pub use self::package::Pkg;
pub use self::composite_spec::CompositeSpec;
pub use self::spec::{ServiceBind, ServiceSpec};
pub use protocol::{ProcessState, StartStyle, Topology, UpdateStrategy};
use super::Sys;
use self::config::CfgRenderer;
use self::hooks::{HOOK_PERMISSIONS, Hook, HookTable};
use SOCKET_CONTEXT;
use error::{Error, Result, SupError};
use fs;
use census::{ServiceFile, CensusRing, ElectionStatus};
use protocol;
use templating::RenderContext;
use util;

static LOGKEY: &'static str = "SR";

lazy_static! {
    static ref HEALTH_CHECK_INTERVAL: Duration = {
        Duration::from_millis(30_000)
    };
}

pub struct Service {
    inner: Arc<RwLock<protocol::Service>>,
    socket: zmq::Socket,
}

impl Service {
    fn new(service: Arc<RwLock<protocol::Service>>) -> Result<Self> {
        let socket = (**SOCKET_CONTEXT).socket(zmq::REP).unwrap();
        socket.connect(service.service_group.as_ref()).unwrap();
        Ok(Service {
            inner: service,
            socket: socket,
        })
    }

    pub fn stop(&self) -> Result<()> {
        // send message to stop service
        unimplemented!()
    }

    pub fn tick(&self) -> Result<()> {
        // send message to let service know that shit changed
        unimplemented!()
    }

    pub fn to_rumor(&self, incarnation: u64) -> ServiceRumor {
        let svc = self.inner.read().unwrap();
        let exported = match svc.cfg.to_exported(&svc.pkg) {
            Ok(exported) => Some(exported),
            Err(err) => {
                outputln!(preamble svc.service_group,
                          "Failed to generate exported cfg for service rumor: {}",
                          Red.bold().paint(format!("{}", err)));
                None
            }
        };
        let mut rumor = ServiceRumor::new(
            svc.sys.member_id.as_str(),
            &svc.pkg.ident,
            &svc.service_group,
            &svc.sys.as_sys_info(),
            exported.as_ref(),
        );
        rumor.set_incarnation(incarnation);
        rumor
    }
}

impl Borrow<ServiceGroup> for Service {
    fn borrow(&self) -> &ServiceGroup {
        &self.inner.service_group
    }
}

impl Deref for Service {
    type Target = protocol::Service;

    fn deref(&self) -> &protocol::Service {
        &self.inner
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.service_group)
    }
}

impl Eq for Service {}

impl Hash for Service {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.service_group.hash(state);
    }
}

impl PartialEq for Service {
    fn eq(&self, other: &Service) -> bool {
        self.inner.service_group == other.inner.service_group
    }
}

pub struct ServiceMgr {
    config_renderer: CfgRenderer,
    census_ring: Arc<RwLock<CensusRing>>,
    health_check: HealthCheck,
    hooks: HookTable,
    initialized: bool,
    inner: Arc<RwLock<protocol::Service>>,
    last_election_status: ElectionStatus,
    last_health_check: Instant,
    launcher: LauncherCli,
    needs_reconfiguration: bool,
    needs_reload: bool,
    pid: Option<Pid>,
    pid_file: PathBuf,
    smoke_check: SmokeCheck,
    state: ProcessState,
    state_entered: Timespec,
}

/// Walk each service and check if it has an updated package installed via the Update Strategy.
/// This updates the Service to point to the new service struct, and then marks it for
/// restarting.
///
/// The run loop's last updated census is a required parameter on this function to inform the
/// main loop that we, ourselves, updated the service counter when we updated ourselves.
// fn check_for_updated_package(&mut self) {
//     if self.updater.check_for_updated_package(service) {
//         self.gossip_latest_service_rumor(&service);
//     }
// }

// ON START
// if let Err(e) = service.create_svc_path() {
//     outputln!(
//         "Can't create directory {}: {}",
//         service.pkg.svc_path.display(),
//         e
//     );
//     outputln!(
//         "If this service is running as non-root, you'll need to create \
//                {} and give the current user write access to it",
//         service.pkg.svc_path.display()
//     );
//     outputln!("{} failed to start", &spec.ident);
//     return;
// }
//
// self.gossip_latest_service_rumor(&service);
// if service.topology == Topology::Leader {
//     self.butterfly.start_election(
//         service.service_group.clone(),
//         0,
//     );
// }

impl ServiceMgr {
    /// Create the service path for this package.
    pub fn init_svc_fs(pkg: &Pkg) -> Result<()> {
        util::users::assert_pkg_user_and_group(&pkg.svc_user, &pkg.svc_group)?;
        create_dir_all(&pkg.svc_path)?;
        // Create Supervisor writable directories
        create_dir_all(fs::svc_hooks_path(&pkg.name))?;
        create_dir_all(fs::svc_logs_path(&pkg.name))?;
        // Create service writable directories
        create_dir_all(&pkg.svc_config_path)?;
        set_owner(&pkg.svc_config_path, &pkg.svc_user, &pkg.svc_group)?;
        set_permissions(&pkg.svc_config_path, 0o700)?;
        create_dir_all(&pkg.svc_data_path)?;
        set_owner(&pkg.svc_data_path, &pkg.svc_user, &pkg.svc_group)?;
        set_permissions(&pkg.svc_data_path, 0o700)?;
        create_dir_all(&pkg.svc_files_path)?;
        set_owner(&pkg.svc_files_path, &pkg.svc_user, &pkg.svc_group)?;
        set_permissions(&pkg.svc_files_path, 0o700)?;
        create_dir_all(&pkg.svc_var_path)?;
        set_owner(&pkg.svc_var_path, &pkg.svc_user, &pkg.svc_group)?;
        set_permissions(&pkg.svc_var_path, 0o700)?;
        remove_symlink(&pkg.svc_static_path)?;
        create_dir_all(&pkg.svc_static_path)?;
        set_owner(&pkg.svc_static_path, &pkg.svc_user, &pkg.svc_group)?;
        set_permissions(&pkg.svc_static_path, 0o700)?;
        Ok(())
    }

    /// Create a new service struct by loading a package install from disk.
    pub fn load(
        sys: Arc<Sys>,
        census: Arc<RwLock<CensusRing>>,
        spec: ServiceSpec,
        organization: Option<&str>,
    ) -> Result<Service> {
        // The package for a spec should already be installed.
        let fs_root_path = Path::new(&*FS_ROOT_PATH);
        let package = PackageInstall::load(&spec.ident, Some(fs_root_path))?;
        spec.validate(&package)?;
        let pkg = Pkg::from_install(package)?;
        let service_group = ServiceGroup::new(
            spec.application_environment.as_ref(),
            &pkg.name,
            spec.group,
            organization,
        )?;
        let config_root = Self::config_root(&pkg, spec.config_from.as_ref());
        let hooks_root = Self::hooks_root(&pkg, spec.config_from.as_ref());
        let launcher = LauncherCli::connect(&mut SOCKET_CONTEXT)?;
        let service = Arc::new(RwLock::new(protocol::Service {
            sys: sys,
            census_ring: census,
            cfg: Cfg::new(&pkg, spec.config_from.as_ref())?,
            config_renderer: CfgRenderer::new(&config_root)?,
            health_check: HealthCheck::default(),
            hooks: HookTable::load(
                &service_group,
                &hooks_root,
                fs::svc_hooks_path(&service_group.service()),
            ),
            initialized: false,
            last_election_status: ElectionStatus::None,
            needs_reload: false,
            needs_reconfiguration: false,
            smoke_check: SmokeCheck::default(),
            last_health_check: Instant::now() - *HEALTH_CHECK_INTERVAL,
            launcher: launcher,
            pid: None,
            pid_file: fs::svc_pid_file(service_group.service()),
            state: ProcessState::Down,
            state_entered: time::get_time(),
            pkg: Arc::new(pkg),
            service_group: Arc::new(service_group),
            spec: Arc::new(spec),
        }));
        Service::from(&service)

        // start thread with service in it
        // return service cli connected to that thred
    }

    fn all_binds_satisfied(svc: &protocol::Service, census_ring: &CensusRing) -> bool {
        let mut ret = true;
        for ref bind in svc.binds().iter() {
            if let Some(group) = census_ring.census_group_for(&bind.service_group) {
                if group.members().iter().all(|m| !m.alive()) {
                    ret = false;
                    outputln!(preamble svc.service_group,
                              "The specified service group '{}' for binding '{}' is present in the \
                               census, but currently has no live members.",
                              Green.bold().paint(format!("{}", bind.service_group)),
                              Green.bold().paint(format!("{}", bind.name)));
                }
            } else {
                ret = false;
                outputln!(preamble svc.service_group,
                          "The specified service group '{}' for binding '{}' is not (yet?) present \
                          in the census data.",
                          Green.bold().paint(format!("{}", bind.service_group)),
                          Green.bold().paint(format!("{}", bind.name)));
            }
        }
        ret
    }

    fn cache_service_file(svc: &protocol::Service, service_file: &ServiceFile) -> bool {
        let file = svc.pkg.svc_files_path.join(&service_file.filename);
        Self::write_cache_file(svc, file, &service_file.body)
    }

    /// Returns the config root given the package and optional config-from path.
    fn config_root<T>(package: &Pkg, config_from: Option<T>) -> PathBuf
    where
        T: AsRef<Path>,
    {
        config_from
            .map(|m| m.as_ref())
            .unwrap_or(&package.path)
            .join("config")
    }

    /// Returns the hooks root given the package and optional config-from path.
    fn hooks_root<T>(package: &Pkg, config_from: Option<T>) -> PathBuf
    where
        T: AsRef<Path>,
    {
        config_from
            .map(|m| m.as_ref())
            .unwrap_or(&package.path)
            .join("hooks")
    }

    /// Write service files from gossip data to disk.
    ///
    /// Returns true if a file was changed, added, or removed, and false if there were no updates.
    fn update_service_files(svc: &protocol::Service, census_ring: &CensusRing) -> bool {
        let census_group = census_ring.census_group_for(&svc.service_group).expect(
            "Service update service files failed; unable to find own service group",
        );
        let mut updated = false;
        for service_file in census_group.changed_service_files() {
            if cache_service_file(svc, &service_file) {
                outputln!(preamble svc.service_group, "Service file updated, {}",
                    service_file.filename);
                updated = true;
            }
        }
        updated
    }

    fn write_cache_file<T>(svc: &protocol::Service, file: T, contents: &[u8]) -> bool
    where
        T: AsRef<Path>,
    {
        let current_checksum = match hash::hash_file(&file) {
            Ok(current_checksum) => current_checksum,
            Err(err) => {
                outputln!(preamble svc.service_group,
                    "Failed to get current checksum for {}, {}",
                    file.as_ref().display(), err);
                String::new()
            }
        };
        let new_checksum = hash::hash_bytes(&contents);
        if new_checksum == current_checksum {
            return false;
        }
        let new_filename = format!("{}.write", file.as_ref().to_string_lossy());
        let mut new_file = match File::create(&new_filename) {
            Ok(new_file) => new_file,
            Err(e) => {
                outputln!(preamble svc.service_group,
                          "Failed to create cache file {}",
                          Red.bold().paint(format!("{}, {}", file.as_ref().display(), e)));
                return false;
            }
        };
        if let Err(e) = new_file.write_all(contents) {
            outputln!(preamble svc.service_group,
                      "Failed to write to cache file {}",
                      Red.bold().paint(format!("{}, {}", file.as_ref().display(), e)));
            return false;
        }
        if let Err(e) = std::fs::rename(&new_filename, &file) {
            outputln!(preamble svc.service_group,
                      "Failed to move cache file {}",
                      Red.bold().paint(format!("{}, {}", file.as_ref().display(), e)));
            return false;
        }
        if let Err(e) = set_owner(&file, &svc.pkg.svc_user, &svc.pkg.svc_group) {
            outputln!(preamble svc.service_group,
                      "Failed to set ownership of cache file {}",
                      Red.bold().paint(format!("{}, {}", file.as_ref().display(), e)));
            return false;
        }
        if let Err(e) = set_permissions(&file, 0o640) {
            outputln!(preamble svc.service_group,
                      "Failed to set permissions on cache file {}",
                      Red.bold().paint(format!("{}, {}", file.as_ref().display(), e)));
            return false;
        }
        true
    }

    pub fn last_state_change(&self) -> Timespec {
        self.state_entered
    }

    pub fn stop(&mut self, svc: &protocol::Service) {
        if self.pid.is_none() {
            return;
        }
        if let Err(err) = self.launcher.terminate(self.pid.unwrap()) {
            outputln!(preamble svc.service_group, "Service stop failed: {}", err);
        }
        self.cleanup_pidfile();
        self.change_state(ProcessState::Down);
    }

    pub fn suitability(&self) -> Option<u64> {
        if !self.initialized {
            return None;
        }
        let svc = self.inner.read().unwrap();
        self.hooks.suitability.as_ref().and_then(|hook| {
            hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password())
        })
    }

    pub fn tick(&mut self) -> bool {
        let svc = self.inner.read().unwrap();
        let census_ring = self.census_ring.read().unwrap();
        // JW TODO: If we change, we need to send ourself to the eventsrv
        // if let Some(member) = census_group.me() {
        //     events.as_ref().map(
        //         |events| events.send_service(member, service),
        //     );
        // }
        if !self.initialized {
            if Self::all_binds_satisfied(&svc, &census_ring) {
                outputln!(preamble svc.service_group, "Waiting for service binds...");
                return false;
            }
        }

        let svc_updated = self.update_templates(&svc, &census_ring);
        if Self::update_service_files(&svc, &census_ring) {
            self.file_updated(&svc);
        }

        match svc.topology() {
            Topology::Standalone => {
                self.execute_hooks(&svc);
            }
            Topology::Leader => {
                {
                    let census_group = census_ring.census_group_for(&svc.service_group).expect(
                        "Service Group's census entry missing from list!",
                    );
                    match census_group.election_status {
                        ElectionStatus::None => {
                            if self.last_election_status != census_group.election_status {
                                outputln!(preamble svc.service_group,
                                          "Waiting to execute hooks; {}",
                                          Yellow.bold().paint("election hasn't started"));
                                self.last_election_status = census_group.election_status;
                            }
                        }
                        ElectionStatus::ElectionInProgress => {
                            if self.last_election_status != census_group.election_status {
                                outputln!(preamble svc.service_group,
                                          "Waiting to execute hooks; {}",
                                          Yellow.bold().paint("election in progress."));
                                self.last_election_status = census_group.election_status;
                            }
                        }
                        ElectionStatus::ElectionNoQuorum => {
                            if self.last_election_status != census_group.election_status {
                                outputln!(preamble svc.service_group,
                                          "Waiting to execute hooks; {}, {}.",
                                          Yellow.bold().paint("election in progress"),
                                          Red.bold().paint("and we have no quorum"));
                                self.last_election_status = census_group.election_status
                            }
                        }
                        ElectionStatus::ElectionFinished => {
                            let leader_id = census_group.leader_id.as_ref().expect(
                                "No leader with finished election",
                            );
                            if self.last_election_status != census_group.election_status {
                                outputln!(preamble svc.service_group,
                                          "Executing hooks; {} is the leader",
                                          Green.bold().paint(leader_id.to_string()));
                                self.last_election_status = census_group.election_status;
                            }
                        }
                    }
                }
                if self.last_election_status == ElectionStatus::ElectionFinished {
                    self.execute_hooks(&svc)
                }
            }
        }
        svc_updated
    }

    /// Replace the package of the running service and restart it's system process.
    pub fn update_package(&mut self, svc: &mut protocol::Service, package: PackageInstall) {
        match Pkg::from_install(package) {
            Ok(pkg) => {
                outputln!(preamble svc.service_group,
                            "Updating service {} to {}", svc.pkg.ident, pkg.ident);
                match CfgRenderer::new(&Self::config_root(&pkg, svc.config_from())) {
                    Ok(renderer) => self.config_renderer = renderer,
                    Err(e) => {
                        outputln!(preamble svc.service_group,
                                  "Failed to load config templates after updating package, {}", e);
                        return;
                    }
                }
                self.hooks = HookTable::load(
                    &svc.service_group,
                    &Self::hooks_root(&pkg, svc.config_from()),
                    fs::svc_hooks_path(svc.service_group.service()),
                );
                svc.pkg = *pkg;
            }
            Err(err) => {
                outputln!(preamble svc.service_group,
                          "Unexpected error while updating package, {}", err);
                return;
            }
        }
        self.stop(&svc);
        self.initialized = false;
    }

    fn cache_health_check(&self, check_result: HealthCheck) {
        unimplemented!()
    }

    fn change_state(&mut self, state: ProcessState) {
        if self.state == state {
            return;
        }
        self.state = state;
        self.state_entered = time::get_time();
    }

    /// Check if the child process is running
    fn check_process(&mut self) -> bool {
        let pid = match self.pid {
            Some(pid) => Some(pid),
            None => {
                if self.pid_file.exists() {
                    Some(read_pid(&self.pid_file).unwrap())
                } else {
                    None
                }
            }
        };
        if let Some(pid) = pid {
            if process::is_alive(pid) {
                self.change_state(ProcessState::Up);
                self.pid = Some(pid);
                return true;
            }
        }
        debug!("Could not find a live process with pid {:?}", self.pid);
        self.change_state(ProcessState::Down);
        self.cleanup_pidfile();
        self.pid = None;
        false
    }

    /// Helper for compiling configuration templates into configuration files.
    fn compile_configuration(&self, svc: &protocol::Service, ctx: &RenderContext) -> bool {
        match self.config_renderer.compile(&svc.pkg, ctx) {
            Ok(true) => {
                outputln!(preamble svc.service_group, "Configuration recompiled");
                true
            }
            Ok(false) => false,
            Err(e) => {
                outputln!(preamble svc.service_group, "Failed to compile configuration: {}", e);
                false
            }
        }
    }

    /// Helper for compiling hook templates into hooks.
    ///
    /// This function will also perform any necessary post-compilation tasks.
    fn compile_hooks(&self, svc: &protocol::Service, ctx: &RenderContext) -> bool {
        let changed = self.hooks.compile(&svc.service_group, ctx);
        if let Some(err) = self.copy_run(svc).err() {
            outputln!(preamble svc.service_group, "Failed to copy run hook: {}", err);
        }
        if changed {
            outputln!(preamble svc.service_group, "Hooks recompiled");
        }
        changed
    }

    // Copy the "run" file to the svc path.
    fn copy_run(&self, svc: &protocol::Service) -> Result<()> {
        let svc_run = svc.pkg.svc_path.join(hooks::RunHook::file_name());
        match self.hooks.run {
            Some(ref hook) => {
                std::fs::copy(hook.path(), &svc_run)?;
                set_permissions(&svc_run.to_str().unwrap(), HOOK_PERMISSIONS)?;
            }
            None => {
                let run = svc.pkg.path.join(hooks::RunHook::file_name());
                match std::fs::metadata(&run) {
                    Ok(_) => {
                        std::fs::copy(&run, &svc_run)?;
                        set_permissions(&svc_run, HOOK_PERMISSIONS)?;
                    }
                    Err(err) => {
                        outputln!(preamble svc.service_group, "Error finding run file: {}", err);
                    }
                }
            }
        }
        Ok(())
    }

    /// Create a PID file for a running service
    fn create_pidfile(&mut self) -> Result<()> {
        match self.pid {
            Some(pid) => {
                debug!(
                    "Creating PID file for child {} -> {:?}",
                    self.pid_file.display(),
                    pid
                );
                let mut f = File::create(&self.pid_file)?;
                write!(f, "{}", pid)?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    fn cleanup_pidfile(&mut self) {
        debug!(
            "Attempting to clean up pid file {}",
            self.pid_file.display()
        );
        match std::fs::remove_file(&self.pid_file) {
            Ok(_) => debug!("Removed pid file"),
            Err(e) => debug!("Error removing pidfile: {}, continuing", e),
        }
    }

    fn execute_hooks(&mut self, svc: &protocol::Service) {
        if !self.initialized {
            if self.check_process() {
                outputln!("Reattached to {}", svc.service_group);
                self.initialized = true;
                return;
            }
            self.initialize(svc);
            if self.initialized {
                self.start(svc);
                self.post_run(svc);
            }
        } else {
            self.check_process();
            if Instant::now().duration_since(self.last_health_check) >= *HEALTH_CHECK_INTERVAL {
                self.run_health_check_hook(svc);
            }

            // NOTE: if you need reconfiguration and you DON'T have a
            // reload script, you're going to restart anyway.
            if self.needs_reload || self.process_down() || self.needs_reconfiguration {
                self.reload(svc);
                if self.needs_reconfiguration {
                    self.reconfigure(svc)
                }
            }
        }
    }

    /// Run file_updated hook if present
    fn file_updated(&self, svc: &protocol::Service) -> bool {
        if self.initialized {
            if let Some(ref hook) = self.hooks.file_updated {
                return hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password());
            }
        }
        false
    }

    /// Run initialization hook if present
    fn initialize(&mut self, svc: &protocol::Service) {
        if self.initialized {
            return;
        }
        outputln!(preamble svc.service_group, "Initializing");
        self.initialized = true;
        if let Some(ref hook) = self.hooks.init {
            self.initialized = hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password())
        }
    }

    fn post_run(&mut self, svc: &protocol::Service) {
        if let Some(ref hook) = self.hooks.post_run {
            hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password());
        }
    }

    fn process_down(&self) -> bool {
        self.state == ProcessState::Down
    }

    /// Run reconfigure hook if present. Return false if it is not present, to trigger default
    /// restart behavior.
    fn reconfigure(&mut self, svc: &protocol::Service) {
        self.needs_reconfiguration = false;
        if let Some(ref hook) = self.hooks.reconfigure {
            hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password());
        }
    }

    fn reload(&mut self, svc: &protocol::Service) {
        self.needs_reload = false;
        if self.process_down() || self.hooks.reload.is_none() {
            if let Some(err) = self.restart(svc).err() {
                outputln!(preamble svc.service_group, "Service restart failed: {}", err);
            }
        } else {
            let hook = self.hooks.reload.as_ref().unwrap();
            hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password());
        }
    }

    /// attempt to remove a symlink in the /svc/run/foo/ directory if
    /// the link exists.
    fn remove_symlink<P>(p: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let p = p.as_ref();
        if !p.exists() {
            return Ok(());
        }
        // note: we're NOT using p.metadata() here as that will follow the
        // symlink, which returns smd.file_type().is_symlink() == false in all cases.
        let smd = p.symlink_metadata()?;
        if smd.file_type().is_symlink() {
            std::fs::remove_file(p)?;
        }
        Ok(())
    }

    /// Helper for constructing a new render context for the service.
    fn render_context<'a>(
        &'a self,
        svc: &'a protocol::Service,
        census: &'a CensusRing,
    ) -> RenderContext<'a> {
        RenderContext::new(
            &svc.service_group,
            &svc.sys,
            &svc.pkg,
            &svc.cfg,
            census,
            svc.binds().iter(),
        )
    }

    fn restart<T>(&mut self, svc: &protocol::Service) -> Result<()>
    where
        T: ToString,
    {
        match self.pid {
            Some(pid) => {
                match self.launcher.restart(pid) {
                    Ok(pid) => {
                        self.pid = Some(pid);
                        self.create_pidfile()?;
                        self.change_state(ProcessState::Up);
                        Ok(())
                    }
                    Err(err) => {
                        self.cleanup_pidfile();
                        self.change_state(ProcessState::Down);
                        Err(sup_error!(Error::Launcher(err)))
                    }
                }
            }
            None => {
                self.start(svc);
                Ok(())
            }
        }
    }

    fn run_health_check_hook(&mut self, svc: &protocol::Service) {
        let check_result = if let Some(ref hook) = self.hooks.health_check {
            hook.run(&svc.service_group, &svc.pkg, svc.svc_encrypted_password())
        } else {
            match self.status(svc) {
                (true, _) => HealthCheck::Ok,
                (false, _) => HealthCheck::Critical,
            }
        };
        self.last_health_check = Instant::now();
        self.cache_health_check(check_result);
    }

    fn start(&mut self, svc: &protocol::Service) {
        outputln!(preamble svc.service_group,
            "Starting service as user={}, group={}",
            &svc.pkg.svc_user, &svc.pkg.svc_group);
        let pid = match self.launcher.spawn(
            svc.service_group.to_string(),
            &svc.pkg.svc_run,
            &svc.pkg.svc_user,
            &svc.pkg.svc_group,
            svc.svc_encrypted_password(),
            svc.pkg.env.clone(),
        ) {
            Ok(pid) => pid,
            Err(err) => {
                outputln!(preamble svc.service_group, "Service start failed: {}", err);
                return;
            }
        };
        self.pid = Some(pid);
        if let Err(err) = self.create_pidfile() {
            outputln!(preamble svc.service_group, "Unable to create pidfile: {}", err);
        }
        self.change_state(ProcessState::Up);
        self.needs_reload = false;
        self.needs_reconfiguration = false;
    }

    fn status(&self, svc: &protocol::Service) -> (bool, String) {
        let status = format!(
            "{}: {} for {}",
            svc.service_group,
            self.state,
            time::get_time() - self.state_entered
        );
        let healthy = match self.state {
            ProcessState::Up => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    /// Compares the current state of the service to the current state of the census ring and
    /// re-renders all templatable content to disk.
    ///
    /// Returns true if any modifications were made.
    fn update_templates(&mut self, svc: &protocol::Service, census_ring: &CensusRing) -> bool {
        let census_group = census_ring.census_group_for(&svc.service_group).expect(
            "Service update failed; unable to find own service group",
        );
        let cfg_updated = svc.cfg.update(census_group);
        if cfg_updated || census_ring.changed() {
            let (reload, reconfigure) = {
                let ctx = self.render_context(svc, census_ring);
                let reload = self.compile_hooks(svc, &ctx);
                let reconfigure = self.compile_configuration(svc, &ctx);
                (reload, reconfigure)
            };
            self.needs_reload = reload;
            self.needs_reconfiguration = reconfigure;
        }
        cfg_updated
    }
}

/// this function wraps create_dir_all so we can give friendly error
/// messages to the user.
fn create_dir_all<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    debug!("Creating dir with subdirs: {:?}", &path.as_ref());
    if let Err(e) = std::fs::create_dir_all(&path) {
        Err(sup_error!(Error::Permissions(
            format!("Can't create {:?}, {}", &path.as_ref(), e),
        )))
    } else {
        Ok(())
    }
}

fn read_pid<T>(pid_file: T) -> Result<Pid>
where
    T: AsRef<Path>,
{
    match File::open(pid_file.as_ref()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => {
                    match line.parse::<Pid>() {
                        Ok(pid) => Ok(pid),
                        Err(_) => Err(sup_error!(
                            Error::PidFileCorrupt(pid_file.as_ref().to_path_buf())
                        )),
                    }
                }
                _ => Err(sup_error!(
                    Error::PidFileCorrupt(pid_file.as_ref().to_path_buf())
                )),
            }
        }
        Err(err) => Err(sup_error!(
            Error::PidFileIO(pid_file.as_ref().to_path_buf(), err)
        )),
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use toml;

    use super::{Topology, UpdateStrategy};
    use error::Error::*;

    #[test]
    fn topology_default() {
        // This should always be the default topology, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(Topology::default(), Topology::Standalone);
    }

    #[test]
    fn topology_from_str() {
        let topology_str = "leader";
        let topology = Topology::from_str(topology_str).unwrap();

        assert_eq!(topology, Topology::Leader);
    }

    #[test]
    fn topology_from_str_invalid() {
        let topology_str = "dope";

        match Topology::from_str(topology_str) {
            Err(e) => {
                match e.err {
                    InvalidTopology(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn topology_to_string() {
        let topology = Topology::Standalone;

        assert_eq!("standalone", topology.to_string())
    }

    #[test]
    fn topology_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: Topology,
        }
        let toml = r#"
            key = "leader"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, Topology::Leader);
    }

    #[test]
    fn topology_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: Topology,
        }
        let data = Data { key: Topology::Leader };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "leader""#))
    }

    #[test]
    fn update_strategy_default() {
        // This should always be the default update strategy, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(UpdateStrategy::default(), UpdateStrategy::None);
    }

    #[test]
    fn update_strategy_from_str() {
        let strategy_str = "at-once";
        let strategy = UpdateStrategy::from_str(strategy_str).unwrap();

        assert_eq!(strategy, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_from_str_invalid() {
        let strategy_str = "dope";

        match UpdateStrategy::from_str(strategy_str) {
            Err(e) => {
                match e.err {
                    InvalidUpdateStrategy(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn update_strategy_to_string() {
        let strategy = UpdateStrategy::AtOnce;

        assert_eq!("at-once", strategy.to_string())
    }

    #[test]
    fn update_strategy_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let toml = r#"
            key = "at-once"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let data = Data { key: UpdateStrategy::AtOnce };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "at-once""#));
    }
}
