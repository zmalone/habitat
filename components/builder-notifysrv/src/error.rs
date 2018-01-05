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

use db;
use hab_core;
use hab_net;
use protobuf;
use protocol;
use postgres;
use r2d2;
use zmq;

pub type SrvResult<T> = Result<T, SrvError>;

#[derive(Debug)]
pub enum SrvError {
    BadPort(String),
    ConnErr(hab_net::conn::ConnErr),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    DbListen(postgres::error::Error),
    HabitatCore(hab_core::Error),
    NetError(hab_net::NetError),
    NotificationCreate(postgres::error::Error),
    Protocol(protocol::ProtocolError),
    Protobuf(protobuf::ProtobufError),
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            SrvError::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            SrvError::ConnErr(ref e) => format!("{}", e),
            SrvError::Db(ref e) => format!("{}", e),
            SrvError::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            SrvError::DbTransactionStart(ref e) => {
                format!("Failed to start database transaction, {}", e)
            }
            SrvError::DbTransactionCommit(ref e) => {
                format!("Failed to commit database transaction, {}", e)
            }
            SrvError::DbListen(ref e) => {
                format!("Error setting up async database event listener, {}", e)
            }
            SrvError::HabitatCore(ref e) => format!("{}", e),
            SrvError::NetError(ref e) => format!("{}", e),
            SrvError::NotificationCreate(ref e) => {
                format!("Error creating notification in database, {}", e)
            }
            SrvError::Protocol(ref e) => format!("{}", e),
            SrvError::Protobuf(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for SrvError {
    fn description(&self) -> &str {
        match *self {
            SrvError::BadPort(_) => {
                "Received an invalid port or a number outside of the valid range."
            }
            SrvError::ConnErr(ref err) => err.description(),
            SrvError::Db(ref err) => err.description(),
            SrvError::DbPoolTimeout(ref err) => err.description(),
            SrvError::DbTransactionStart(ref err) => err.description(),
            SrvError::DbTransactionCommit(ref err) => err.description(),
            SrvError::DbListen(ref err) => err.description(),
            SrvError::HabitatCore(ref err) => err.description(),
            SrvError::NetError(ref err) => err.description(),
            SrvError::NotificationCreate(ref err) => err.description(),
            SrvError::Protocol(ref err) => err.description(),
            SrvError::Protobuf(ref err) => err.description(),
        }
    }
}

impl From<r2d2::GetTimeout> for SrvError {
    fn from(err: r2d2::GetTimeout) -> Self {
        SrvError::DbPoolTimeout(err)
    }
}

impl From<hab_core::Error> for SrvError {
    fn from(err: hab_core::Error) -> Self {
        SrvError::HabitatCore(err)
    }
}

impl From<hab_net::NetError> for SrvError {
    fn from(err: hab_net::NetError) -> Self {
        SrvError::NetError(err)
    }
}

impl From<hab_net::conn::ConnErr> for SrvError {
    fn from(err: hab_net::conn::ConnErr) -> Self {
        SrvError::ConnErr(err)
    }
}

impl From<db::error::Error> for SrvError {
    fn from(err: db::error::Error) -> Self {
        SrvError::Db(err)
    }
}

impl From<protobuf::ProtobufError> for SrvError {
    fn from(err: protobuf::ProtobufError) -> Self {
        SrvError::Protobuf(err)
    }
}

impl From<protocol::ProtocolError> for SrvError {
    fn from(err: protocol::ProtocolError) -> Self {
        SrvError::Protocol(err)
    }
}

impl From<zmq::Error> for SrvError {
    fn from(err: zmq::Error) -> Self {
        SrvError::from(hab_net::conn::ConnErr::from(err))
    }
}
