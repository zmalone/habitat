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

use std::error;
use std::fmt;
use std::result;
use std::str::FromStr;

use serde::{Serialize, Serializer};

pub use message::notifysrv::*;
use message::Routable;

#[derive(Debug)]
pub enum Error {
    BadNotificationCategory,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadNotificationCategory => "Bad Notification Category",
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadNotificationCategory => "Notification category cannot be parsed",
        }
    }
}

impl Default for NotificationCategory {
    fn default() -> NotificationCategory {
        NotificationCategory::Info
    }
}

impl Serialize for NotificationCategory {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self as u64 {
            1 => serializer.serialize_str("info"),
            2 => serializer.serialize_str("error"),
            _ => panic!("Unexpected enum value"),
        }
    }
}

impl FromStr for NotificationCategory {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {

        match value.to_lowercase().as_ref() {
            "info" => Ok(NotificationCategory::Info),
            "error" => Ok(NotificationCategory::Error),
            _ => Err(Error::BadNotificationCategory),
        }
    }
}

impl fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            NotificationCategory::Info => "info",
            NotificationCategory::Error => "error",
        };
        write!(f, "{}", value)
    }
}

impl Routable for NotificationCreate {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_notification().get_origin_id())
    }
}
