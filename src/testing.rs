use super::*;
use diesel::result::Error;
use std::sync::LazyLock;
use testcontainers_modules::{
    postgres,
    testcontainers::{runners::SyncRunner, Container, ImageExt},
};

static TEST_DATABASE_CONTAINER: LazyLock<Container<postgres::Postgres>> = LazyLock::new(|| {
    postgres::Postgres::default()
        .with_tag("16")
        .start()
        .unwrap()
});

pub fn get_db_url() -> String {
    let container = &*TEST_DATABASE_CONTAINER;
    let host_port = container.get_host_port_ipv4(5432).unwrap();
    let host = container.get_host().unwrap();
    format!("postgres://postgres:postgres@{host}:{host_port}/postgres")
}

pub fn get_db_pool() -> db::DbPool {
    db::new_pool(&get_db_url()).unwrap()
}

pub fn get_db_connection() -> db::DbPooledConnection {
    get_db_pool().get().unwrap()
}

pub fn with_db<F>(f: F)
where
    F: FnOnce(&mut PgConnection),
{
    get_db_connection().test_transaction::<_, Error, _>(|conn| {
        db::migrate(conn).ok();
        f(conn);
        Ok(())
    })
}
