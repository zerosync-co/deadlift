use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new(
                "/static",
                "./examples/camara-project/static",
            ))
            .service(actix_files::Files::new(
                "/pkg",
                "./examples/camara-project/create_qod_session/pkg",
            ))
            .default_service(web::route().to(fallback))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn fallback() -> Result<NamedFile> {
    Ok(NamedFile::open(
        "./examples/camara-project/static/index.html",
    )?)
}
