// Copyright 2018 foundationdb-rs developers, https://github.com/bluejekyll/foundationdb-rs/graphs/contributors
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use foundationdb_sys as fdb;
use std;
use std::sync::Arc;

use cluster::*;
use error::{self, *};
use options;
use transaction::*;

#[derive(Clone)]
pub struct Database {
    cluster: Cluster,
    inner: Arc<DatabaseInner>,
}
impl Database {
    pub(crate) fn new(cluster: Cluster, db: *mut fdb::FDBDatabase) -> Self {
        let inner = Arc::new(DatabaseInner::new(db));
        Self { cluster, inner }
    }

    pub fn set_option(&self, opt: options::DatabaseOption) -> Result<()> {
        unsafe { opt.apply(self.inner.inner) }
    }

    pub fn create_trx(&self) -> Result<Transaction> {
        unsafe {
            let mut trx: *mut fdb::FDBTransaction = std::ptr::null_mut();
            error::eval(fdb::fdb_database_create_transaction(
                self.inner.inner,
                &mut trx as *mut _,
            ))?;
            Ok(Transaction::new(self.clone(), trx))
        }
    }
}

struct DatabaseInner {
    inner: *mut fdb::FDBDatabase,
}
impl DatabaseInner {
    fn new(inner: *mut fdb::FDBDatabase) -> Self {
        Self { inner }
    }
}
impl Drop for DatabaseInner {
    fn drop(&mut self) {
        unsafe {
            fdb::fdb_database_destroy(self.inner);
        }
    }
}