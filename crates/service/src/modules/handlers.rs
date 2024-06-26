use actix_multipart::form::{bytes::Bytes, text::Text, MultipartForm};
use actix_web::{get, post, web, HttpResponse, Responder};
use extism::{Manifest, PluginBuilder, Wasm, WasmMetadata};
use serde_json::Value;

use super::module::Module;

#[derive(MultipartForm)]
struct CreateModuleParams {
    binary: Bytes,
    title: Text<String>,
    description: Option<Text<String>>,
    subject: Text<String>,
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
            form.subject.0,
        )
    })
    .await
    {
        Ok(Ok(hash)) => HttpResponse::Created().body(hash),
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[get("/modules")]
pub async fn list_modules_handler() -> impl Responder {
    match web::block(Module::list).await {
        Ok(Ok(items)) => HttpResponse::Ok().json(items),
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

    let manifest = Manifest::new([Wasm::Data {
        data: binary,
        meta: WasmMetadata::default(),
    }])
    .with_allowed_host("TODO");

    let mut plugin = PluginBuilder::new(manifest)
        .with_wasi(true)
        .build()
        .unwrap();

    let output = plugin.call::<Value, Value>("_main", input).unwrap();
    HttpResponse::Ok().json(output)
}
