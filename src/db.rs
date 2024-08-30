use diesel::{
    migration::{self, MigrationVersion}, r2d2::{ConnectionManager, Pool, PoolError, PooledConnection}, PgConnection
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Create a new connection pool from a database connection URL.
pub fn new_pool(database_url: &str) -> Result<DbPool, PoolError> {
    Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url))
}

/// Run database migrations.
pub fn migrate(
    connection: &mut PgConnection,
) -> migration::Result<Vec<MigrationVersion<'_>>> {
    connection.run_pending_migrations(MIGRATIONS)
}
