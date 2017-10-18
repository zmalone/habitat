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

use std::fmt;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use core::channel::STABLE_CHANNEL;
use core::os::process::Pid;
use core::package::PackageIdent;
use core::service::{ApplicationEnvironment, ServiceGroup};
use core::url::DEFAULT_BLDR_URL;
use time::Timespec;
use toml;

static DEFAULT_GROUP: &'static str = "default";

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
pub enum ElectionStatus {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

impl Default for ElectionStatus {
    fn default() -> ElectionStatus {
        ElectionStatus::None
    }
}

impl fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            ElectionStatus::ElectionInProgress => "in-progress",
            ElectionStatus::ElectionNoQuorum => "no-quorum",
            ElectionStatus::ElectionFinished => "finished",
            ElectionStatus::None => "none",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Serialize)]
pub enum HealthCheck {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl Default for HealthCheck {
    fn default() -> HealthCheck {
        HealthCheck::Unknown
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ProcessState {
    Down,
    Up,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match *self {
            ProcessState::Down => "down",
            ProcessState::Up => "up",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
pub enum SmokeCheck {
    Ok,
    Failed(i32),
    Pending,
}

impl Default for SmokeCheck {
    fn default() -> SmokeCheck {
        SmokeCheck::Pending
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum StartStyle {
    Persistent,
    Transient,
}

impl Default for StartStyle {
    fn default() -> StartStyle {
        StartStyle::Transient
    }
}

impl fmt::Display for StartStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            StartStyle::Persistent => "persistent",
            StartStyle::Transient => "transient",
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Topology {
    Standalone,
    Leader,
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum UpdateStrategy {
    None,
    AtOnce,
    Rolling,
}

impl UpdateStrategy {
    fn as_str(&self) -> &str {
        match *self {
            UpdateStrategy::None => "none",
            UpdateStrategy::AtOnce => "at-once",
            UpdateStrategy::Rolling => "rolling",
        }
    }
}

impl fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for UpdateStrategy {
    fn default() -> UpdateStrategy {
        UpdateStrategy::None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub cfg: Cfg,
    pub election_status: ElectionStatus,
    pub health_check: HealthCheck,
    pub pid: Option<Pid>,
    pub pkg: Pkg,
    pub service_group: ServiceGroup,
    pub smoke_check: SmokeCheck,
    pub state: ProcessState,
    // JW TODO: Timespec
    pub state_entered: u64,
    pub sys: Sys,
    spec: ServiceSpec,
}

impl Service {
    pub fn ident(&self) -> &PackageIdent {
        &self.spec.ident
    }

    pub fn group(&self) -> &str {
        &self.spec.group
    }

    pub fn application_environment(&self) -> Option<&ApplicationEnvironment> {
        self.spec.application_environment.as_ref()
    }

    pub fn bldr_url(&self) -> &str {
        &self.spec.bldr_url
    }

    pub fn channel(&self) -> &str {
        &self.spec.channel
    }

    pub fn topology(&self) -> Topology {
        self.spec.topology
    }

    pub fn update_strategy(&self) -> UpdateStrategy {
        self.spec.update_strategy
    }

    pub fn binds(&self) -> &[ServiceBind] {
        self.spec.binds.as_slice()
    }

    pub fn config_from(&self) -> Option<&Path> {
        self.spec.config_from.as_ref().map(PathBuf::as_path)
    }

    pub fn desired_state(&self) -> ProcessState {
        self.spec.desired_state
    }

    pub fn start_style(&self) -> StartStyle {
        self.spec.start_style
    }

    pub fn svc_encrypted_password(&self) -> Option<&str> {
        self.spec.svc_encrypted_password.as_ref().map(
            String::as_str,
        )
    }

    pub fn composite(&self) -> Option<&str> {
        self.spec.composite.as_ref().map(String::as_str)
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.service_group, self.pkg.ident)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ServiceBind {
    pub name: String,
    pub service_group: ServiceGroup,
}

impl fmt::Display for ServiceBind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.service_group)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ServiceSpec {
    pub ident: PackageIdent,
    pub group: String,
    pub application_environment: Option<ApplicationEnvironment>,
    pub bldr_url: String,
    pub channel: String,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<ServiceBind>,
    pub config_from: Option<PathBuf>,
    pub desired_state: ProcessState,
    pub start_style: StartStyle,
    pub svc_encrypted_password: Option<String>,
    pub composite: Option<String>,
}

impl Default for ServiceSpec {
    fn default() -> Self {
        ServiceSpec {
            ident: PackageIdent::default(),
            group: DEFAULT_GROUP.to_string(),
            application_environment: None,
            bldr_url: DEFAULT_BLDR_URL.to_string(),
            channel: STABLE_CHANNEL.to_string(),
            topology: Topology::default(),
            update_strategy: UpdateStrategy::default(),
            binds: Vec::default(),
            config_from: None,
            desired_state: ProcessState::Up,
            start_style: StartStyle::default(),
            svc_encrypted_password: None,
            composite: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sys {
    pub version: String,
    pub member_id: String,
    pub ip: IpAddr,
    pub hostname: String,
    pub gossip_ip: IpAddr,
    pub gossip_port: u16,
    pub http_gateway_ip: IpAddr,
    pub http_gateway_port: u16,
    pub permanent: bool,
}
