use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub mod db;
pub mod graphql;
pub mod models;
pub mod schema;

#[cfg(test)]
mod testing;

pub fn establish_connection() -> ConnectionResult<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

#[cfg(test)]
mod test {
    use crate::testing::*;

    #[test]
    fn connection_can_be_established() {
        get_db_connection();
    }

    #[test]
    fn migrations_can_be_run() {
        with_db_conn(|_conn| {});
    }
}
