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

mod handlers;

use hab_net::app::prelude::*;
use protocol::notifysrv::*;

use config::Config;
use data_store::DataStore;
use error::{SrvError, SrvResult};

lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable<NotifySrv> = {
        let mut map = DispatchTable::new();
        map.register(NotificationCreate::descriptor_static(None),
            handlers::create_notification);
        map
    };
}

#[derive(Clone)]
pub struct ServerState {
    datastore: DataStore,
}

impl ServerState {
    fn new(cfg: Config) -> SrvResult<Self> {
        Ok(ServerState {
            datastore: DataStore::new(&cfg.datastore, cfg.app.shards.unwrap())?,
        })
    }
}

impl AppState for ServerState {
    type Error = SrvError;
    type InitState = Self;

    fn build(init_state: Self::InitState) -> SrvResult<Self> {
        Ok(init_state)
    }
}

struct NotifySrv;
impl Dispatcher for NotifySrv {
    const APP_NAME: &'static str = "builder-notifysrv";
    const PROTOCOL: Protocol = Protocol::NotifySrv;

    type Config = Config;
    type Error = SrvError;
    type State = ServerState;

    fn app_init(
        config: Self::Config,
        _: Arc<String>,
    ) -> SrvResult<<Self::State as AppState>::InitState> {
        let state = ServerState::new(config)?;
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), SrvError> {
    app_start::<NotifySrv>(config)
}

pub fn migrate(config: Config) -> SrvResult<()> {
    let ds = DataStore::new(&config.datastore, config.app.shards.unwrap())?;
    ds.setup()
}
