use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use deadlift_service::{modules::handlers::*, services::*, workflows::handlers::*};

// TODO-- refactor to use axum!!

fn cors_middleware() -> Cors {
    Cors::default()
        .allow_any_origin()
        // .send_wildcard()
        .supports_credentials()
        .allowed_headers(vec![
            "Accept",
            "Accept-Encoding",
            "Authorization",
            "Cache-Control",
            "Content-Length",
            "Content-Type",
            "Origin",
            "User-Agent",
            "X-CSRF-Token",
            "X-Requested-With",
        ])
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .expose_headers(vec!["X-Total-Results-Count"])
        .disable_vary_header()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    db::init(); // TODO-- db functions should be async so that you don't have to wrap them with web::block all the time

    HttpServer::new(|| {
        App::new()
            .wrap(cors_middleware())
            .service(status_handler)
            .service(
                web::scope("/api/v1")
                    .service(create_module_handler)
                    .service(list_modules_handler)
                    .service(execute_module_handler)
                    .service(create_workflow_handler)
                    .service(execute_workflow_handler),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/status")]
async fn status_handler() -> impl Responder {
    HttpResponse::NoContent()
}
