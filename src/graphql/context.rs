use std::sync::{Arc, Mutex};

use crate::db::DbPooledConnection;

pub struct Context {
    pub db_conn_mutex: Arc<Mutex<DbPooledConnection>>,
}

impl Context {
    pub fn with_db_conn<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut DbPooledConnection) -> T,
    {
        let mut conn = self.db_conn_mutex.lock().unwrap();
        f(&mut *conn)
    }
}

impl juniper::Context for Context {}
