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

use db::migration::Migrator;

use error::SrvResult;

pub fn migrate(migrator: &mut Migrator) -> SrvResult<()> {
    migrator.migrate(
        "notifysrv",
        r#"CREATE SEQUENCE IF NOT EXISTS notification_id_seq;"#,
    )?;
    migrator.migrate(
        "notifysrv",
        r#"CREATE TABLE IF NOT EXISTS notifications (
                    id bigint PRIMARY KEY DEFAULT next_id_v1('notification_id_seq'),
                    origin_id bigint NOT NULL,
                    account_id bigint NOT NULL,
                    category text NOT NULL,
                    data text NOT NULL,
                    created_at timestamptz DEFAULT now(),
                    updated_at timestamptz
             )"#,
    )?;
    migrator.migrate(
        "notifysrv",
        r#"CREATE OR REPLACE FUNCTION insert_notification_v1 (
                     n_origin_id bigint,
                     n_account_id bigint,
                     n_category text,
                     n_data text
                 ) RETURNS SETOF notifications AS $$
                        INSERT INTO notifications (origin_id, account_id, category, data)
                        VALUES (n_origin_id, n_account_id, n_category, n_data)
                        RETURNING *
                 $$ LANGUAGE SQL VOLATILE"#,
    )?;
    Ok(())
}
