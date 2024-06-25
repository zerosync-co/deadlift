use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

type DbManager = ConnectionManager<SqliteConnection>;

pub type DbConnection = PooledConnection<DbManager>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

static POOL: once_cell::sync::Lazy<Pool<DbManager>> = once_cell::sync::Lazy::new(|| {
    let manager = ConnectionManager::<SqliteConnection>::new("database.sqlite");
    Pool::<DbManager>::new(manager).expect("db pool")
});

pub fn init() {
    let mut conn = connection().expect("db connection");

    conn.batch_execute("PRAGMA foreign_keys = ON;")
        .expect("enable foreign keys");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("run pending migrations");
}

pub fn connection() -> Result<DbConnection, diesel::result::Error> {
    POOL.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    })
}
