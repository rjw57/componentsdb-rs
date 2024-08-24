use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{env, error::Error};

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn migrate_database(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod test {
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
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub fn within_test_transaction<F>(f: F)
    where
        F: FnOnce(&mut PgConnection),
    {
        establish_test_database_connection().test_transaction::<_, Error, _>(|conn| {
            migrate_database(conn).unwrap();
            f(conn);
            Ok(())
        })
    }

    #[test]
    fn connection_can_be_established() {
        establish_test_database_connection();
    }

    #[test]
    fn migrations_can_be_run() {
        within_test_transaction(|_conn| {});
    }
}
