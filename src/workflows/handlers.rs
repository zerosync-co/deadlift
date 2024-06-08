use actix_web::{post, web, HttpResponse, Responder};
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
    let pipeline_str = match serde_json::to_string(&params.pipeline) {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    match web::block(move || Workflow::create(params.name, params.description, pipeline_str)).await
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
    let workflow = match Workflow::find_by_id(path.into_inner()) {
        Ok(v) => v,
        Err(diesel::NotFound) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::BadRequest().finish(),
    };

    let pipeline_iterator = workflow.get_pipeline();
    let mut engine = crate::modules::engine::Engine::new().unwrap(); // FIXME--

    let mut output = Value::default();
    for hash in pipeline_iterator {
        let binary = match Module::get_binary_by_hash(hash) {
            Ok(v) => v,
            Err(diesel::NotFound) => return HttpResponse::NotFound().finish(),
            _ => return HttpResponse::BadRequest().finish(),
        };

        output = engine.run(&binary, &input).unwrap(); // FIXME--
    }

    HttpResponse::Ok().json(output)
}
