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

//! Tracks rumors for distribution.
//!
//! Each rumor is represented by a `RumorKey`, which has a unique key and a "kind", which
//! represents what "kind" of rumor it is (for example, a "member").
//!
//! New rumors need to implement the `From` trait for `RumorKey`, and then can track the arrival of
//! new rumors, and dispatch them according to their `kind`.

pub mod dat_file;
pub mod departure;
pub mod election;
pub mod heat;
pub mod service;
pub mod service_config;
pub mod service_file;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use bytes::BytesMut;
use lmdb;
use lmdb::traits::*;
use prost::Message as ProstMessage;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub use self::departure::Departure;
pub use self::election::{Election, ElectionUpdate};
pub use self::service::Service;
pub use self::service_config::ServiceConfig;
pub use self::service_file::ServiceFile;
use error::{Error, Result};
use member::Membership;
pub use protocol::newscast::{Rumor as ProtoRumor, RumorPayload, RumorType};
use protocol::{FromProto, Message};

#[derive(Debug, Clone, Serialize)]
pub enum RumorKind {
    Departure(Departure),
    Election(Election),
    ElectionUpdate(ElectionUpdate),
    Membership(Membership),
    Service(Service),
    ServiceConfig(ServiceConfig),
    ServiceFile(ServiceFile),
}

impl From<RumorKind> for RumorPayload {
    fn from(value: RumorKind) -> Self {
        match value {
            RumorKind::Departure(departure) => RumorPayload::Departure(departure.into()),
            RumorKind::Election(election) => RumorPayload::Election(election.into()),
            RumorKind::ElectionUpdate(election) => RumorPayload::Election(election.into()),
            RumorKind::Membership(membership) => RumorPayload::Member(membership.into()),
            RumorKind::Service(service) => RumorPayload::Service(service.into()),
            RumorKind::ServiceConfig(service_config) => {
                RumorPayload::ServiceConfig(service_config.into())
            }
            RumorKind::ServiceFile(service_file) => RumorPayload::ServiceFile(service_file.into()),
        }
    }
}

/// The description of a `RumorKey`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RumorKey {
    pub kind: RumorType,
    pub id: String,
    pub key: String,
}

impl RumorKey {
    pub fn new<A, B>(kind: RumorType, id: A, key: B) -> RumorKey
    where
        A: ToString,
        B: ToString,
    {
        RumorKey {
            kind: kind,
            id: id.to_string(),
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> String {
        if self.key.len() > 0 {
            format!("{}-{}", self.id, self.key)
        } else {
            format!("{}", self.id)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MergeResult<T: Rumor> {
    ShareExisting,
    ShareNew(T),
    StopSharing,
}

/// A representation of a Rumor; implemented by all the concrete types we share as rumors. The
/// exception is the Membership rumor, since it's not actually a rumor in the same vein.
pub trait Rumor: Message<ProtoRumor> + fmt::Debug + Sized {
    fn kind(&self) -> RumorType;
    fn key(&self) -> &str;
    fn id(&self) -> &str;
    fn merge(&self, other: Self) -> MergeResult<Self>;
}

impl<'a, T: Rumor> From<&'a T> for RumorKey {
    fn from(rumor: &'a T) -> RumorKey {
        RumorKey::new(rumor.kind(), rumor.id(), rumor.key())
    }
}

/// Storage for Rumors. It takes a rumor and stores it according to the member that produced it,
/// and the service group it is related to.
///
/// Generic over the type of rumor it stores.
#[derive(Debug, Clone)]
pub struct RumorStore<T: Rumor> {
    name: String,
    db: Arc<lmdb::Database<'static>>,
    phantom: PhantomData<T>,
    //pub list: Arc<RwLock<HashMap<String, HashMap<String, T>>>>,
    update_counter: Arc<AtomicUsize>,
}

//impl<T> Deref for RumorStore<T>
//where
//    T: Rumor,
//{
//    type Target = RwLock<HashMap<String, HashMap<String, T>>>;
//
//    fn deref(&self) -> &Self::Target {
//        &*self.list
//    }
//}

impl<T> Serialize for RumorStore<T>
where
    T: Rumor + lmdb::traits::AsLmdbBytes + lmdb::traits::FromLmdbBytes,
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("rumor_store", 2)?;
        strukt.serialize_field("name", &*(self.name))?;
        //       strukt.serialize_field("list", &*(self.list.read().unwrap()))?;
        strukt.serialize_field("update_counter", &self.get_update_counter())?;
        strukt.end()
    }
}

impl<T> RumorStore<T>
where
    T: Rumor + lmdb::traits::AsLmdbBytes + lmdb::traits::FromLmdbBytes,
{
    /// Create a new RumorStore for the given type. Allows you to initialize the counter to a
    /// pre-set value. Useful mainly in testing.
    pub fn new(name: String, counter: usize, lmdb_env: Arc<lmdb::Environment>) -> RumorStore<T> {
        let rumor_db_options = lmdb::DatabaseOptions::create_map::<str>();
        let db = lmdb::Database::open(lmdb_env.clone(), Some(&name), &rumor_db_options).unwrap();
        RumorStore {
            name: name,
            db: Arc::new(db),
            phantom: PhantomData,
            //          list: Arc::new(RwLock::new(HashMap::new())),
            update_counter: Arc::new(AtomicUsize::new(counter)),
        }
    }

    /// Clear all rumors and reset update counter of RumorStore.
    pub fn clear(&self) -> usize {
        let txn = lmdb::WriteTransaction::new(self.db.env()).expect("failed to get txn");
        {
            let mut access = txn.access();
            access.clear_db(&self.db).expect("Clearing database failed");
        }
        txn.commit().expect("Transaction commit failed");
        self.update_counter.swap(0, Ordering::Relaxed)
    }

    fn db_key(&self, key: &str, member_id: &str) -> String {
        format!("{}:::{}", key, member_id)
    }

    pub fn encode(&self, key: &str, member_id: &str) -> Result<Vec<u8>> {
        let key = self.db_key(key, member_id);
        let txn = lmdb::ReadTransaction::new(self.db.env())?;
        let access = txn.access();
        match access.get::<str, T>(&self.db, key.as_ref()).to_opt()? {
            Some(r) => {
                println!("{:?}", r);
                r.write_to_bytes()
            }
            None => Err(Error::NonExistentRumor(
                String::from(member_id),
                String::from(key),
            )),
        }
        // let list = self.list.read().expect("Rumor store lock poisoned");
        // match list.get(key).and_then(|l| l.get(member_id)) {
        //     Some(rumor) => rumor.clone().write_to_bytes(),
        //     None => Err(Error::NonExistentRumor(
        //         String::from(member_id),
        //         String::from(key),
        //     )),
        // }
    }

    pub fn get_update_counter(&self) -> usize {
        self.update_counter.load(Ordering::Relaxed)
    }

    // TODO: Can these two functions stay gone? If so, it will be great, because this is an
    // expensive operation.

    /// Returns the count of all rumors in this RumorStore.
    //pub fn len(&self) -> usize {
    //    self.list
    //        .read()
    //        .expect("Rumor store lock poisoned")
    //        .values()
    //        .map(|member| member.len())
    //        .sum()
    //}

    /// Returns the count of all rumors in the rumor store for the given member's key.
    //    pub fn len_for_key(&self, key: &str) -> usize {
    //    let list = self.list.read().expect("Rumor store lock poisoned");
    //    list.get(key).map_or(0, |r| r.len())
    //}

    /// Insert a rumor into the Rumor Store. Returns true if the value didn't exist or if it was
    /// mutated; if nothing changed, returns false.
    pub fn insert(&self, rumor: T) -> bool {
        // LMDB
        let txn = lmdb::WriteTransaction::new(self.db.env()).expect("failed to get txn");
        let key = self.db_key(rumor.key(), rumor.id());
        let updated_bool = {
            let merge_result = {
                let mut access = txn.access();
                let rumor_option = match access.get::<str, T>(&self.db, key.as_ref()).to_opt() {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Failed to get data from LMDB about rumor: {}", e);
                        None
                    }
                };

                match rumor_option {
                    Some(r) => r.merge(rumor),
                    None => MergeResult::ShareNew::<T>(rumor),
                }
            };

            let updated_bool = {
                let mut access = txn.access();
                match merge_result {
                    MergeResult::StopSharing => false,
                    MergeResult::ShareExisting => true,
                    MergeResult::ShareNew::<T>(new_rumor) => {
                        access
                            .put::<str, T>(
                                &self.db,
                                key.as_ref(),
                                &new_rumor,
                                lmdb::put::Flags::empty(),
                            ).expect("failed to get db");
                        true
                    }
                }
            };

            if updated_bool {
                self.increment_update_counter();
            }
            updated_bool
        };
        txn.commit().unwrap();
        updated_bool

        // Old skool
        // let mut list = self.list.write().expect("Rumor store lock poisoned");
        // let rumors = list
        //     .entry(String::from(rumor.key()))
        //     .or_insert(HashMap::new());
        // // Result reveals if there was a change so we can increment the counter if needed.
        // let result = match rumors.entry(rumor.id().into()) {
        //     Entry::Occupied(mut entry) => entry.get_mut().merge(rumor),
        //     Entry::Vacant(entry) => {
        //         entry.insert(rumor);
        //         true
        //     }
        // };
        // if result {
        //     self.increment_update_counter();
        // }
        //
    }

    pub fn remove(&self, key: &str, id: &str) {
        let txn = lmdb::WriteTransaction::new(self.db.env()).expect("failed to get txn");
        let key = self.db_key(key, id);
        {
            let mut access = txn.access();
            access
                .del_key::<str>(&self.db, key.as_ref())
                .to_opt()
                .expect("Failed to remove key");
        }
        txn.commit().expect("Transaction failed");
    }

    pub fn with_all_rumors<F>(&self, mut with_closure: F)
    where
        F: FnMut(&T),
    {
        let txn = lmdb::ReadTransaction::new(self.db.env()).expect("failed to get txn");
        let mut cursor = txn.cursor(self.db.clone()).expect("failed to get cursor");
        let access = txn.access();
        let first_result = cursor
            .first::<str, T>(&access)
            .to_opt()
            .expect("failed to get first election");

        match first_result {
            Some((_k, rumor)) => with_closure(rumor),
            None => return,
        }

        loop {
            let next_result = cursor
                .next::<str, T>(&access)
                .to_opt()
                .expect("Error getting next value");
            match next_result {
                Some((_k, rumor)) => {
                    with_closure(rumor);
                }
                None => return,
            }
        }
    }

    // pub fn with_keys<F>(&self, mut with_closure: F)
    // where
    //     F: FnMut((&String, &HashMap<String, T>)),
    // {
    //     let list = self.list.read().expect("Rumor store lock poisoned");
    //     for x in list.iter() {
    //         with_closure(x);
    //     }
    // }

    pub fn with_rumors<F>(&self, key: &str, mut with_closure: F)
    where
        F: FnMut(&T),
    {
        let txn = lmdb::ReadTransaction::new(self.db.env()).expect("failed to get txn");
        let mut cursor = txn.cursor(self.db.clone()).expect("failed to get cursor");
        let access = txn.access();
        let first_result = cursor
            .seek_range_k::<str, T>(&access, key)
            .to_opt()
            .expect("failed to seek for range k");

        match first_result {
            Some((_k, rumor)) => with_closure(rumor),
            None => return,
        }

        loop {
            let next_result = cursor
                .next::<str, T>(&access)
                .to_opt()
                .expect("Error getting next value");
            match next_result {
                Some((_k, rumor)) => {
                    if rumor.key() == key {
                        with_closure(rumor);
                    } else {
                        return;
                    }
                }
                None => return,
            }
        }

        //let list = self.list.read().expect("Rumor store lock poisoned");
        //if list.contains_key(key) {
        //    for x in list.get(key).unwrap().values() {
        //        with_closure(x);
        //    }
        //}
    }

    pub fn with_rumor<F>(&self, key: &str, member_id: &str, mut with_closure: F)
    where
        F: FnMut(Option<&T>),
    {
        let txn = lmdb::ReadTransaction::new(self.db.env()).expect("failed to get txn");
        let key = self.db_key(key, member_id);
        let access = txn.access();
        match access.get::<str, T>(&self.db, &key).to_opt() {
            Ok(opt) => {
                with_closure(opt);
            }
            Err(e) => {
                error!("Error trying to lookup a rumor in lmdb: {}", e);
                return;
            }
        }
        //let list = self.list.read().expect("Rumor store lock poisoned");
        //with_closure(list.get(key).and_then(|r| r.get(member_id)));
    }

    pub fn contains_rumor(&self, key: &str, id: &str) -> bool {
        let txn = lmdb::ReadTransaction::new(self.db.env()).expect("failed to get txn");
        let key = self.db_key(key, id);
        error!("key: {}", key);
        let access = txn.access();
        match access.get::<str, T>(&self.db, &key).to_opt() {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!("Error trying to lookup a rumor in lmdb: {}", e);
                false
            }
        }
    }

    /// Increment the update counter for this store.
    ///
    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    fn increment_update_counter(&self) {
        self.update_counter.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RumorEnvelope {
    pub type_: RumorType,
    pub from_id: String,
    pub kind: RumorKind,
}

impl RumorEnvelope {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let proto = ProtoRumor::decode(bytes)?;
        let type_ = RumorType::from_i32(proto.type_).ok_or(Error::ProtocolMismatch("type"))?;
        let from_id = proto
            .from_id
            .clone()
            .ok_or(Error::ProtocolMismatch("from-id"))?;
        let kind = match type_ {
            RumorType::Departure => RumorKind::Departure(Departure::from_proto(proto)?),
            RumorType::Election => RumorKind::Election(Election::from_proto(proto)?),
            RumorType::ElectionUpdate => {
                RumorKind::ElectionUpdate(ElectionUpdate::from_proto(proto)?)
            }
            RumorType::Member => RumorKind::Membership(Membership::from_proto(proto)?),
            RumorType::Service => RumorKind::Service(Service::from_proto(proto)?),
            RumorType::ServiceConfig => RumorKind::ServiceConfig(ServiceConfig::from_proto(proto)?),
            RumorType::ServiceFile => RumorKind::ServiceFile(ServiceFile::from_proto(proto)?),
            RumorType::Fake | RumorType::Fake2 => panic!("fake rumor"),
        };
        Ok(RumorEnvelope {
            type_: type_,
            from_id: from_id,
            kind: kind,
        })
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        let proto: ProtoRumor = self.into();
        let mut buf = BytesMut::with_capacity(proto.encoded_len());
        proto.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

impl From<RumorEnvelope> for ProtoRumor {
    fn from(value: RumorEnvelope) -> ProtoRumor {
        ProtoRumor {
            type_: value.type_ as i32,
            tag: vec![],
            from_id: Some(value.from_id),
            payload: Some(value.kind.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{mem, slice};

    use lmdb::traits::*;
    use uuid::Uuid;

    use error::Result;
    use protocol::{self, newscast};
    use rumor::{MergeResult, Rumor, RumorType};

    #[repr(C)]
    #[derive(Clone, Debug, Serialize)]
    struct FakeRumor {
        pub id: String,
        pub key: String,
    }

    impl AsLmdbBytes for FakeRumor {
        fn as_lmdb_bytes(&self) -> &[u8] {
            unsafe {
                slice::from_raw_parts(
                    (self as *const FakeRumor) as *const u8,
                    mem::size_of::<FakeRumor>(),
                )
            }
        }
    }

    impl FromLmdbBytes for FakeRumor {
        fn from_lmdb_bytes(bytes: &[u8]) -> ::std::result::Result<&FakeRumor, String> {
            let bytes_ptr: *const u8 = bytes.as_ptr();
            let rumor_ptr: *const FakeRumor = bytes_ptr as *const FakeRumor;
            let thing: &FakeRumor = unsafe { &*rumor_ptr };
            Ok(thing)
        }
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor {
                id: format!("{}", Uuid::new_v4().to_simple_ref()),
                key: String::from("fakerton"),
            }
        }
    }

    #[repr(C)]
    #[derive(Clone, Debug, Serialize)]
    struct TrumpRumor {
        pub id: String,
        pub key: String,
    }

    impl AsLmdbBytes for TrumpRumor {
        fn as_lmdb_bytes(&self) -> &[u8] {
            unsafe {
                slice::from_raw_parts(
                    (self as *const TrumpRumor) as *const u8,
                    mem::size_of::<TrumpRumor>(),
                )
            }
        }
    }

    impl FromLmdbBytes for TrumpRumor {
        fn from_lmdb_bytes(bytes: &[u8]) -> ::std::result::Result<&TrumpRumor, String> {
            let bytes_ptr: *const u8 = bytes.as_ptr();
            let rumor_ptr: *const TrumpRumor = bytes_ptr as *const TrumpRumor;
            let thing: &TrumpRumor = unsafe { &*rumor_ptr };
            Ok(thing)
        }
    }

    impl Rumor for FakeRumor {
        fn kind(&self) -> RumorType {
            RumorType::Fake
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn merge(&self, mut _other: FakeRumor) -> MergeResult<FakeRumor> {
            MergeResult::StopSharing
        }
    }

    impl protocol::FromProto<newscast::Rumor> for FakeRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> {
            Ok(FakeRumor::default())
        }
    }

    impl From<FakeRumor> for newscast::Rumor {
        fn from(_other: FakeRumor) -> newscast::Rumor {
            newscast::Rumor::default()
        }
    }

    impl protocol::Message<newscast::Rumor> for FakeRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> {
            Ok(FakeRumor::default())
        }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    impl Default for TrumpRumor {
        fn default() -> TrumpRumor {
            TrumpRumor {
                id: format!("{}", Uuid::new_v4().to_simple_ref()),
                key: String::from("fakerton"),
            }
        }
    }

    impl Rumor for TrumpRumor {
        fn kind(&self) -> RumorType {
            RumorType::Fake2
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn merge(&self, mut _other: TrumpRumor) -> MergeResult<TrumpRumor> {
            MergeResult::StopSharing
        }
    }

    impl protocol::FromProto<newscast::Rumor> for TrumpRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> {
            Ok(TrumpRumor::default())
        }
    }

    impl From<TrumpRumor> for newscast::Rumor {
        fn from(_other: TrumpRumor) -> newscast::Rumor {
            newscast::Rumor::default()
        }
    }

    impl protocol::Message<newscast::Rumor> for TrumpRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> {
            Ok(TrumpRumor::default())
        }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    mod rumor_store {
        use super::FakeRumor;
        use lmdb;
        use rumor::Rumor;
        use rumor::RumorStore;
        use std::sync::Arc;
        use std::usize;
        use tempdir::TempDir;

        fn create_new_lmdb_env() -> Arc<lmdb::Environment> {
            let tmp_dir = TempDir::new("rumorstore-test").unwrap();
            unsafe {
                let mut lmdb_env = lmdb::EnvBuilder::new().unwrap();
                lmdb_env.set_mapsize(3000000000).unwrap();
                lmdb_env.set_maxdbs(20).unwrap();
                Arc::new(
                    lmdb_env
                        .open(
                            tmp_dir.path().to_str().unwrap(),
                            lmdb::open::Flags::empty(),
                            0o600,
                        ).unwrap(),
                )
            }
        }

        fn create_rumor_store() -> RumorStore<FakeRumor> {
            RumorStore::new(String::from("fakerumor"), 0, create_new_lmdb_env())
        }

        #[test]
        fn update_counter() {
            let rs = create_rumor_store();
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 1);
        }

        #[test]
        fn update_counter_overflows_safely() {
            let rs: RumorStore<FakeRumor> =
                RumorStore::new(String::from("fakerumor"), usize::MAX, create_new_lmdb_env());
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 0);
        }

        #[test]
        fn insert_adds_rumor_when_empty() {
            let rs = create_rumor_store();
            let f = FakeRumor::default();
            assert!(rs.insert(f));
            assert_eq!(rs.get_update_counter(), 1);
        }

        #[test]
        fn insert_adds_multiple_rumors_for_same_key() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f1_id = String::from(f1.id());
            let f2 = FakeRumor::default();
            let f2_id = String::from(f2.id());

            assert!(rs.insert(f1));
            assert!(rs.insert(f2));
            assert_eq!(rs.get_update_counter(), 2);
            //assert_eq!(
            //    rs.list
            //        .read()
            //        .unwrap()
            //        .get(&key)
            //        .unwrap()
            //        .get(&f1_id)
            //        .unwrap()
            //        .id,
            //    f1_id
            //);
            //assert_eq!(
            //    rs.list
            //        .read()
            //        .unwrap()
            //        .get(&key)
            //        .unwrap()
            //        .get(&f2_id)
            //        .unwrap()
            //        .id,
            //    f2_id
            //);
        }

        #[test]
        fn insert_adds_multiple_members() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f2 = FakeRumor::default();
            assert!(rs.insert(f1));
            assert!(rs.insert(f2));
            assert_eq!(rs.get_update_counter(), 2);
        }

        #[test]
        fn insert_returns_false_on_no_changes() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let f2 = f1.clone();
            assert!(rs.insert(f1));
            assert_eq!(rs.insert(f2), false);
        }

        #[test]
        fn with_rumor_calls_closure_with_rumor() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let member_id = f1.id.clone();
            let key = f1.key.clone();
            rs.insert(f1);
            rs.with_rumor(&key, &member_id, |o| assert_eq!(o.unwrap().id, member_id));
        }

        #[test]
        fn with_rumor_calls_closure_with_none_if_rumor_missing() {
            let rs = create_rumor_store();
            rs.with_rumor("bar", "foo", |o| assert!(o.is_none()));
        }
    }
}
