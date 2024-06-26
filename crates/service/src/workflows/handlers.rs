use actix_web::{post, web, HttpResponse, Responder};
use extism::{Manifest, PluginBuilder, Wasm, WasmMetadata};
use serde::Deserialize;
use serde_json::Value;

use crate::modules::module::Module;

use super::workflow::Workflow;

#[derive(Deserialize)]
struct CreateWorkflowParams {
    name: String,
    description: Option<String>,
    pipeline: Vec<String>,
}

#[post("/workflows")]
pub async fn create_workflow_handler(
    web::Json(params): web::Json<CreateWorkflowParams>,
) -> impl Responder {
    match web::block(move || Workflow::create(params.name, params.description, params.pipeline))
        .await
    {
        Ok(Ok(id)) => HttpResponse::Created().body(id.to_string()),
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[post("/workflows/{workflow_id}/execute")]
pub async fn execute_workflow_handler(
    path: web::Path<i32>,
    web::Json(input): web::Json<Value>,
) -> impl Responder {
    let workflow_id = path.into_inner();
    let workflow = match Workflow::find_by_id(workflow_id) {
        Ok(v) => v,
        Err(diesel::NotFound) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::BadRequest().finish(),
    };

    let pipeline_iterator = workflow.get_pipeline();

    let mut current_value = input.clone();
    for workflow_module in pipeline_iterator {
        let binary = Module::get_binary_by_hash(workflow_module.get_hash()).unwrap(); // FIXME--
        let manifest = Manifest::new([Wasm::Data {
            data: binary,
            meta: WasmMetadata::default(),
        }])
        .with_allowed_host("TODO");

        let mut plugin = PluginBuilder::new(manifest)
            .with_wasi(true)
            .build()
            .unwrap();

        current_value = plugin.call::<Value, Value>("_main", current_value).unwrap();
    }

    println!(
        "workflow id: {}; input: {}; output: {}",
        workflow_id, input, current_value
    );

    HttpResponse::Ok().json(current_value)
}
