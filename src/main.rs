use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("database.sqlite");
    let pool = Pool::<ConnectionManager<SqliteConnection>>::new(manager).expect("db pool");
    let mut conn = pool.get().expect("db connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("run pending migrations");

    HttpServer::new(|| App::new().service(status_handler))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/status")]
async fn status_handler() -> impl Responder {
    HttpResponse::NoContent()
}
