use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use modules::handlers::create_module_handler;

mod modules;
mod schema;
mod services;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    services::db::init();

    HttpServer::new(|| {
        App::new()
            .service(status_handler)
            .service(web::scope("/api/v1").service(create_module_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/status")]
async fn status_handler() -> impl Responder {
    HttpResponse::NoContent()
}
