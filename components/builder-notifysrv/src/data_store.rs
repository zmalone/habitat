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

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

use db::config::{DataStoreCfg, ShardId};
use db::error::{Error as DbError, Result as DbResult};
use db::migration::Migrator;
use db::pool::Pool;
use hab_net::conn::{RouteClient, RouteConn};
use hab_net::{ErrCode, NetError};
use hab_core::package::PackageIdent;
use postgres::rows::Rows;
use protocol::{originsrv, sessionsrv, jobsrv};
use protocol::net::NetOk;
use protocol::originsrv::Pageable;
use postgres;
use protobuf;

use error::{SrvError, SrvResult};
use migrations;

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
}

impl DataStore {
    pub fn new(cfg: &DataStoreCfg, shards: Vec<ShardId>) -> SrvResult<DataStore> {
        let pool = Pool::new(&cfg, shards)?;
        let ap = pool.clone();
        Ok(DataStore { pool: pool })
    }

    pub fn from_pool(pool: Pool, _: Arc<String>) -> SrvResult<DataStore> {
        Ok(DataStore { pool: pool })
    }

    pub fn setup(&self) -> SrvResult<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        migrations::notifications::migrate(&mut migrator)?;

        migrator.finish()?;

        Ok(())
    }
}
