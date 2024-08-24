use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{env, error::Error};

pub mod models;
pub mod schema;

#[cfg(test)]
mod testing;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn migrate_database(
    connection: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() -> ConnectionResult<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

#[cfg(test)]
mod test {
    use crate::testing::*;

    #[test]
    fn connection_can_be_established() {
        establish_test_database_connection();
    }

    #[test]
    fn migrations_can_be_run() {
        with_db(|_conn| {});
    }
}
