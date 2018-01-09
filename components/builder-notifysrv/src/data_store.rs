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

use std::sync::Arc;

use db::config::{DataStoreCfg, ShardId};
use db::migration::Migrator;
use db::pool::Pool;
use protocol::notifysrv;
use postgres;

use error::{SrvError, SrvResult};
use migrations;

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
}

impl DataStore {
    pub fn new(cfg: &DataStoreCfg, shards: Vec<ShardId>) -> SrvResult<DataStore> {
        let pool = Pool::new(&cfg, shards).map_err(SrvError::Db)?;
        Ok(DataStore { pool: pool })
    }

    pub fn from_pool(pool: Pool, _: Arc<String>) -> SrvResult<DataStore> {
        Ok(DataStore { pool: pool })
    }

    pub fn setup(&self) -> SrvResult<()> {
        let conn = self.pool.get_raw().map_err(SrvError::Db)?;
        let xact = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup().map_err(SrvError::Db)?;

        migrations::notifications::migrate(&mut migrator)?;

        migrator.finish().map_err(SrvError::Db)?;

        Ok(())
    }

    pub fn create_notification(
        &self,
        notification_create: &notifysrv::NotificationCreate,
    ) -> SrvResult<notifysrv::Notification> {
        let conn = self.pool.get(notification_create).map_err(SrvError::Db)?;
        let notification = notification_create.get_notification();
        let mut cat = notification.get_category().to_string();

        if cat.is_empty() {
            cat = notifysrv::NotificationCategory::default().to_string();
        }

        let rows = conn.query(
            "SELECT * FROM insert_notification_v1($1, $2, $3, $4)",
            &[
                &(notification.get_origin_id() as i64),
                &(notification.get_account_id() as i64),
                &cat,
                &notification.get_data(),
            ],
        ).map_err(SrvError::NotificationCreate)?;
        let row = rows.get(0);
        let notification = self.row_to_notification(row)?;
        Ok(notification)
    }

    fn row_to_notification(&self, row: postgres::rows::Row) -> SrvResult<notifysrv::Notification> {
        let mut notification = notifysrv::Notification::new();
        let id: i64 = row.get("id");
        let origin_id: i64 = row.get("origin_id");
        let account_id: i64 = row.get("account_id");
        notification.set_id(id as u64);
        notification.set_origin_id(origin_id as u64);
        notification.set_account_id(account_id as u64);

        let cat: String = row.get("category");
        let new_cat: notifysrv::NotificationCategory =
            cat.parse().map_err(SrvError::UnknownNotificationCategory)?;
        notification.set_category(new_cat);

        notification.set_data(row.get("data"));
        Ok(notification)
    }
}
