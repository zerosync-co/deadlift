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
            form.description.and_then(|v| Some(v.0)),
        )
    })
    .await
    {
        Ok(Ok(hash)) => HttpResponse::Created().body(hash),
        _ => HttpResponse::BadRequest().finish(),
    }
}
