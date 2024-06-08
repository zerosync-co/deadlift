use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};

use super::module::Module;

#[derive(MultipartForm)]
struct CreateModuleParams {
    binary: Bytes,
    title: Text<String>,
    description: Option<Text<String>>,
}

#[post("/modules")]
pub async fn create_module_handler(
    MultipartForm(form): MultipartForm<CreateModuleParams>,
) -> impl Responder {
    match web::block(move || {
        Module::create(
            form.binary.data.into(),
            form.title.0,
            form.description.map(|v| v.0),
        )
    })
    .await
    {
        Ok(Ok(hash)) => HttpResponse::Created().body(hash),
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[post("/modules/{module_hash}/execute")]
pub async fn execute_module_handler(
    path: web::Path<String>,
    web::Json(input): web::Json<serde_json::Value>,
) -> impl Responder {
    let binary = match Module::get_binary_by_hash(path.into_inner().as_str()) {
        Ok(v) => v,
        Err(diesel::NotFound) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::BadRequest().finish(),
    };

    let output = tokio::task::spawn_blocking(move || {
        let mut engine = crate::modules::engine::Engine::new().unwrap(); // FIXME--
        engine.run(&binary, &input).unwrap() // FIXME--
    })
    .await
    .unwrap(); // FIXME--
    HttpResponse::Ok().json(output)
}
