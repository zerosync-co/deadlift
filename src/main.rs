use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(status_handler))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/status")]
async fn status_handler() -> impl Responder {
    HttpResponse::NoContent()
}
