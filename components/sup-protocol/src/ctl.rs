// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Specific request, responses, and types used to specifically communicate with the Supervisor's
//! Control Gateway.
//!
//! Note: See `protocols/ctl.proto` for type level documentation for generated types.

include!(concat!(env!("OUT_DIR"), "/sup.ctl.rs"));

use std::fmt;
use std::net::{Ipv4Addr, SocketAddr};

use message;

/// Default listening port for the CtlGateway listener.
pub const DEFAULT_PORT: u16 = 9632;

impl message::MessageStatic for NetProgress {
    const MESSAGE_ID: &'static str = "NetProgress";
}

impl message::MessageStatic for Handshake {
    const MESSAGE_ID: &'static str = "Handshake";
}

impl message::MessageStatic for ServiceBindList {
    const MESSAGE_ID: &'static str = "ServiceBindList";
}

impl message::MessageStatic for SupDepart {
    const MESSAGE_ID: &'static str = "SupDepart";
}

impl message::MessageStatic for SvcFilePut {
    const MESSAGE_ID: &'static str = "SvcFilePut";
}

impl message::MessageStatic for SvcGetDefaultCfg {
    const MESSAGE_ID: &'static str = "SvcGetDefaultCfg";
}

impl message::MessageStatic for SvcValidateCfg {
    const MESSAGE_ID: &'static str = "SvcValidateCfg";
}

impl message::MessageStatic for SvcSetCfg {
    const MESSAGE_ID: &'static str = "SvcSetCfg";
}

impl message::MessageStatic for SvcLoad {
    const MESSAGE_ID: &'static str = "SvcLoad";
}

impl message::MessageStatic for SvcUnload {
    const MESSAGE_ID: &'static str = "SvcUnload";
}

impl message::MessageStatic for SvcStart {
    const MESSAGE_ID: &'static str = "SvcStart";
}

impl message::MessageStatic for SvcStop {
    const MESSAGE_ID: &'static str = "SvcStop";
}

impl message::MessageStatic for SvcStatus {
    const MESSAGE_ID: &'static str = "SvcStatus";
}

impl message::MessageStatic for ConsoleLine {
    const MESSAGE_ID: &'static str = "ConsoleLine";
}

/// Return a SocketAddr with the default listening address and port.
pub fn default_addr() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT))
}

impl fmt::Display for ConsoleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}
