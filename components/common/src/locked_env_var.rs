// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Unit test helpers for dealing with environment variables.
//!
//! A lot of our functionality can change depending on how certain
//! environment variables are set. This complicates unit testing,
//! because Rust runs test cases in parallel on separate threads, but
//! environment variables are shared across threads. Thus, one test
//! may modify an environment variable for itself while simultaneously
//! clobbering any similar changes being made concurrently by another
//! test.
//!
//! We can run all tests using only a single thread, but this
//! needlessly slows down the entire test suite for what may be as few
//! as two test cases.
//!
//! This module provides types and macros to use in tests to create a
//! global lock on individual environment variables. For tests that
//! depend on a given environment variable, you should declare a locked
//! variable using the `locked_env_var!` macro _outside_ of any
//! individual test case. Then, in each test case, you can obtain the
//! lock to the variable using the lock function created by the
//! macro. This will provide a reference to the locked environment
//! variable, ensuring that the current test is the only one with
//! access to it. Changes may be made using the `set` method of the
//! lock; any value that the variable had prior to the test is
//! remembered and set back when the lock is dropped.
//!
//! This does, of course, depend on the test author taking care to set
//! up the locking infrastructure, and you're on the honor system to
//! not try and modify the variable outside of the bounds of this
//! locking paradigm. Once the locks are in place, however, only the
//! tests that need access to the locked variable will be serialized,
//! leaving the rest of the tests to proceed in parallel.

use std::env;
use std::ffi::{OsStr, OsString};
use std::sync::MutexGuard;

/// Models an exclusive "honor system" lock on a single environment variable.
pub struct LockedEnvVar {
    /// The checked-out lock for the variable.
    lock: MutexGuard<'static, String>,
    /// The original value of the environment variable, prior to any
    /// modifications through this lock.
    ///
    /// `Some` means a value was set when this struct was created,
    /// while `None` means that the variable was not present.
    original_value: Option<OsString>,
}

impl LockedEnvVar {
    /// Create a new lock. Users should not call this directly, but
    /// use locking function generated by the `locked_env_var!` macro.
    ///
    /// The current value of the variable is recorded at the time of
    /// creation; it will be reset when the lock is dropped.
    pub fn new(lock: MutexGuard<'static, String>) -> Self {
        let original = match env::var(&*lock) {
            Ok(val) => Some(OsString::from(val)),
            Err(env::VarError::NotPresent) => None,
            Err(env::VarError::NotUnicode(os_string)) => Some(os_string),
        };
        LockedEnvVar {
            lock,
            original_value: original,
        }
    }

    /// Set the locked environment variable to `value`.
    pub fn set<V>(&self, value: V)
    where
        V: AsRef<OsStr>,
    {
        env::set_var(&*self.lock, value.as_ref());
    }

    /// Unsets an environment variable.
    pub fn unset(&self) {
        env::remove_var(&*self.lock);
    }
}

impl Drop for LockedEnvVar {
    fn drop(&mut self) {
        match self.original_value {
            Some(ref val) => {
                env::set_var(&*self.lock, val);
            }
            None => {
                env::remove_var(&*self.lock);
            }
        }
    }
}

/// Create a static thread-safe mutex for accessing a named
/// environment variable.
///
/// `lock_fn` is the name of the function to create to actually check
/// out this lock. You have to provide it explicitly because Rust's
/// macros are not able to generate identifiers at this time.
#[macro_export]
macro_rules! locked_env_var {
    ($env_var_name:ident, $lock_fn:ident) => {
        lazy_static! {
            static ref $env_var_name: ::std::sync::Arc<::std::sync::Mutex<String>> =
                ::std::sync::Arc::new(::std::sync::Mutex::new(String::from(stringify!(
                    $env_var_name
                ))));
        }

        fn $lock_fn() -> $crate::locked_env_var::LockedEnvVar {
            // Yup, we're ignoring poisoned mutexes. We're not
            // actually changing the contents of the mutex, just using
            // it to serialize access.
            //
            // Furthermore, if a test using the lock fails, that's a
            // panic! That would end up failing any tests that were
            // run afterwards.
            let lock = match $env_var_name.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            $crate::locked_env_var::LockedEnvVar::new(lock)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::VarError;
    use std::thread;

    #[test]
    fn initially_unset_value_is_unset_after_drop() {
        // Don't use this variable for any other tests, because we're
        // going to be poking at it outside of the lock to verify the
        // macro and types behave properly!
        locked_env_var!(HAB_TESTING_LOCKED_ENV_VAR_INITIALLY_UNSET, lock_var);

        assert_eq!(
            env::var("HAB_TESTING_LOCKED_ENV_VAR_INITIALLY_UNSET"),
            Err(env::VarError::NotPresent)
        );

        {
            let lock = lock_var();
            lock.set("foo");
            assert_eq!(
                env::var("HAB_TESTING_LOCKED_ENV_VAR_INITIALLY_UNSET"),
                Ok(String::from("foo"))
            );
        }

        assert_eq!(
            env::var("HAB_TESTING_LOCKED_ENV_VAR_INITIALLY_UNSET"),
            Err(env::VarError::NotPresent)
        );
    }

    #[test]
    fn original_value_is_retained_across_multiple_modifications() {
        // Don't use this variable for any other tests, because we're
        // going to be poking at it outside of the lock to verify the
        // macro and types behave properly!
        locked_env_var!(HAB_TESTING_LOCKED_ENV_VAR, lock_var);

        env::set_var("HAB_TESTING_LOCKED_ENV_VAR", "original_value");

        {
            let lock = lock_var();
            lock.set("foo");
            assert_eq!(
                env::var("HAB_TESTING_LOCKED_ENV_VAR"),
                Ok(String::from("foo"))
            );
            lock.set("bar");
            assert_eq!(
                env::var("HAB_TESTING_LOCKED_ENV_VAR"),
                Ok(String::from("bar"))
            );
            lock.set("foobar");
            assert_eq!(
                env::var("HAB_TESTING_LOCKED_ENV_VAR"),
                Ok(String::from("foobar"))
            );
            lock.unset();
            assert_eq!(
                env::var("HAB_TESTING_LOCKED_ENV_VAR"),
                Err(VarError::NotPresent)
            );
        }

        assert_eq!(
            env::var("HAB_TESTING_LOCKED_ENV_VAR"),
            Ok(String::from("original_value"))
        );
    }

    #[test]
    fn can_recover_from_poisoned_mutex() {
        locked_env_var!(HAB_TESTING_LOCKED_ENV_VAR_POISONED, lock_var);

        // Poison the lock
        let _ = thread::Builder::new()
            .name("testing-locked-env-var-panic".into())
            .spawn(move || -> () {
                let _lock = lock_var();
                panic!("This is an intentional panic; it's OK");
            }).expect("Couldn't spawn thread!")
            .join();

        // We should still be able to do something with it; otherwise
        // any test that used this variable and failed would fail any
        // other test that ran after it.
        let lock = lock_var();
        lock.set("poisoned foo");

        assert_eq!(
            env::var("HAB_TESTING_LOCKED_ENV_VAR_POISONED"),
            Ok(String::from("poisoned foo"))
        );
    }
}
