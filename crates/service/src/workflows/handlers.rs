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
    let pipeline_str = params.pipeline.join(",");

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
    let workflow_id = path.into_inner();
    let workflow = match Workflow::find_by_id(workflow_id) {
        Ok(v) => v,
        Err(diesel::NotFound) => return HttpResponse::NotFound().finish(),
        _ => return HttpResponse::BadRequest().finish(),
    };

    let cloned_input = input.clone();
    let output = tokio::task::spawn_blocking(move || {
        let pipeline_iterator = workflow.get_pipeline();
        let mut engine = crate::modules::engine::Engine::new().unwrap(); // FIXME--

        let mut current_value = cloned_input;
        for hash in pipeline_iterator {
            let binary = Module::get_binary_by_hash(hash).unwrap(); // FIXME--
            current_value = engine.run(&binary, &current_value).unwrap(); // FIXME--
        }

        current_value
    })
    .await
    .unwrap(); // FIXME--

    println!("workflow id: {}; input: {}; output: {}", workflow_id, input, output);

    HttpResponse::Ok().json(output)
}
