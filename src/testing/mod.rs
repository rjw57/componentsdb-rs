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

pub fn establish_test_database_connection() -> PgConnection {
    let container = &*TEST_DATABASE_CONTAINER;
    let host_port = container.get_host_port_ipv4(5432).unwrap();
    let host = container.get_host().unwrap();
    let database_url = format!("postgres://postgres:postgres@{host}:{host_port}/postgres");
    PgConnection::establish(&database_url).unwrap()
}

pub fn with_db<F>(f: F)
where
    F: FnOnce(&mut PgConnection),
{
    establish_test_database_connection().test_transaction::<_, Error, _>(|conn| {
        migrate_database(conn).unwrap();
        f(conn);
        Ok(())
    })
}
