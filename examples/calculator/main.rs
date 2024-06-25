use reqwest::*;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let base_url = "http://localhost:8080";

    // check status
    let status_res = client
        .get(format!("{}/status", base_url))
        .send()
        .await
        .expect("status res");
    assert_eq!(status_res.status(), 204);

    // create add_ten module
    let add_ten_bytes = read_module_bytes("add_ten");
    let add_ten_binary_part = multipart::Part::bytes(add_ten_bytes);
    let add_ten_form = multipart::Form::new()
        .text("title", "add_ten")
        .part("binary", add_ten_binary_part)
        .text("subject", "deadlift.modules.ingest.add-ten");
    let create_add_ten_module_res = client
        .post(format!("{}/api/v1/modules", base_url))
        .multipart(add_ten_form)
        .send()
        .await
        .expect("create add_ten module res");
    assert_eq!(create_add_ten_module_res.status(), 201);
    let add_ten_module_hash = create_add_ten_module_res
        .text()
        .await
        .expect("add_ten module hash");
    println!("add_ten module hash: {}", add_ten_module_hash);

    // test add_ten module
    let execute_add_ten_module_res = client
        .post(format!(
            "{}/api/v1/modules/{}/execute",
            base_url, add_ten_module_hash
        ))
        .json(&Value::from(5))
        .send()
        .await
        .expect("execute add_ten module res");
    assert_eq!(execute_add_ten_module_res.status(), 200);
    assert_eq!(
        execute_add_ten_module_res
            .json::<Value>()
            .await
            .unwrap_or_default(),
        Value::from(15)
    );

    // create multiply_by_five module
    let multiply_by_five_bytes = read_module_bytes("multiply_by_five");
    let multiply_by_five_binary_part = multipart::Part::bytes(multiply_by_five_bytes);
    let multiply_by_five_form = multipart::Form::new()
        .text("title", "multiply_by_five")
        .part("binary", multiply_by_five_binary_part)
        .text("subject", "deadlift.modules.ingest.multiply-by-five");
    let create_multiply_by_five_module_res = client
        .post(format!("{}/api/v1/modules", base_url))
        .multipart(multiply_by_five_form)
        .send()
        .await
        .expect("create multiply_by_five module res");
    assert_eq!(
        create_multiply_by_five_module_res.status(),
        201,
        "{}",
        create_multiply_by_five_module_res
            .text()
            .await
            .unwrap_or_default()
    );
    let multiply_by_five_module_hash = create_multiply_by_five_module_res
        .text()
        .await
        .expect("multiply_by_five module hash");
    println!(
        "multiply_by_five module hash: {}",
        multiply_by_five_module_hash
    );

    // test multiply_by_five module
    let execute_multiply_by_five_module_res = client
        .post(format!(
            "{}/api/v1/modules/{}/execute",
            base_url, multiply_by_five_module_hash
        ))
        .json(&Value::from(5))
        .send()
        .await
        .expect("execute multiply_by_five module res");
    assert_eq!(execute_multiply_by_five_module_res.status(), 200);
    assert_eq!(
        execute_multiply_by_five_module_res
            .json::<Value>()
            .await
            .unwrap_or_default(),
        Value::from(25)
    );

    // create workflow
    let create_workflow_params = serde_json::json!({
        "name": "calculator",
        "pipeline": ["deadlift.modules.ingest.add-ten", "deadlift.modules.ingest.multiply-by-five"]
    });
    let create_workflow_res = client
        .post(format!("{}/api/v1/workflows", base_url))
        .json(&create_workflow_params)
        .send()
        .await
        .expect("create workflow res");
    assert_eq!(create_workflow_res.status(), 201);
    let workflow_id = create_workflow_res.text().await.expect("workflow id");
    println!("calculator workflow id: {}", workflow_id);

    // execute workflow
    let execute_workflow_res = client
        .post(format!(
            "{}/api/v1/workflows/{}/execute",
            base_url, workflow_id
        ))
        .json(&Value::from(5))
        .send()
        .await
        .expect("execute workflow res");
    assert_eq!(execute_workflow_res.status(), 200);
    assert_eq!(
        execute_workflow_res
            .json::<Value>()
            .await
            .unwrap_or_default(),
        Value::from(75)
    );
}

fn read_module_bytes(name: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/examples/calculator/{}/target/wasm32-wasi/release/{}.wasm",
        env!("CARGO_MANIFEST_DIR"), name, name
    )).expect(format!("failed to read {} module bytes; did you build the module with 'cargo build --release --target wasm32-wasi'?", name).as_str())
}
