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

use hab_net::app::prelude::*;
use postgres::error::Error as PostgresError;
use protocol::net;
use protocol::originsrv as proto;

use super::ServerState;
use error::{SrvError, SrvResult};

pub fn create_notification(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let mut msg = req.parse::<proto::NotificationCreate>()?;
    match state.datastore.create_notification(&mut msg) {
        Ok(Some(ref notification)) => conn.route_reply(req, notification)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:notification-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}
