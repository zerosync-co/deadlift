use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{Array, Function, Object, Reflect},
    ReadableStreamDefaultReader, Response, ResponseInit,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn fetch(url: &str, opts: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);

    #[wasm_bindgen(js_namespace = process, js_name = env)]
    static ENV: JsValue;
}

#[wasm_bindgen(module = "nats")]
extern "C" {
    #[wasm_bindgen(catch, js_name = connect)]
    async fn create_nats_connection() -> Result<JsValue, JsValue>;
}

#[allow(dead_code)]
fn get_env_var(key: &str) -> Option<String> {
    let value = js_sys::Reflect::get(&ENV, &JsValue::from_str(key)).ok()?;
    value.as_string()
}

async fn read_all_chunks(reader: ReadableStreamDefaultReader) -> Result<Vec<u8>, JsValue> {
    let mut data = Vec::new();

    loop {
        let result = JsFuture::from(reader.read()).await?;
        let result_obj = result.unchecked_into::<Object>();
        if Reflect::get(&result_obj, &JsValue::from_str("done"))?
            .as_bool()
            .unwrap_or(false)
        {
            break;
        }

        let value = Reflect::get(&result_obj, &JsValue::from_str("value"))?;
        if let Some(buffer) = value.dyn_ref::<js_sys::Uint8Array>() {
            data.extend(buffer.to_vec());
        }
    }

    Ok(data)
}

pub async fn publish_to_nats(subject: &str, message: &str) -> Result<(), JsValue> {
    let nats_conn = create_nats_connection().await?;

    let js_val_subject = JsValue::from_str(subject);
    let js_val_message = JsValue::from_str(message);

    let args = Array::new();
    args.push(&js_val_subject);
    args.push(&js_val_message);

    let publish_function =
        Reflect::get(&nats_conn, &JsValue::from_str("publish"))?.dyn_into::<Function>()?;

    Reflect::apply(&publish_function, &nats_conn, &args)?;

    log("successfully published to nats");
    Ok(())
}

#[wasm_bindgen]
pub async fn wasm_fetch(url: String, opts: JsValue) -> Result<Response, JsValue> {
    let result = fetch(&url, opts.clone()).await?;

    let response = result.dyn_into::<Response>()?;

    if let Some(stream) = response.body() {
        let reader = ReadableStreamDefaultReader::new(&stream)?;

        let mut response_body = read_all_chunks(reader).await?;

        let response_json = serde_json::from_slice::<serde_json::Value>(response_body.as_slice())
            .unwrap_or_default();

        let request_opts_json = serde_wasm_bindgen::from_value::<serde_json::Value>(opts).unwrap();

        let deadlift_output = serde_json::json!({
            "request": {
                "url": url,
                "opts": request_opts_json
            },
            "response": {
                "url": response.url(),
                "body": response_json
            }
        });

        if let Err(e) = publish_to_nats(
            "deadlift.modules.ingest.fetch-realworld",
            deadlift_output.to_string().as_str(),
        )
        .await
        {
            log(&format!("failed to publish to nats; {e:?}"));
        }

        let mut options = ResponseInit::new();
        options.status(response.status());
        options.headers(&response.headers());

        return Response::new_with_opt_u8_array_and_init(Some(&mut response_body), &options);
    }

    Ok(response)
}
