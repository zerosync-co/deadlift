use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct RealworldFetchPayload {
    request: RequestInfo,
    response: ResponseInfo,
}

#[derive(Deserialize, Serialize)]
struct RequestInfo {
    opts: Value,
    url: String,
}

#[derive(Deserialize, Serialize)]
struct ResponseInfo {
    body: Value,
    url: String,
}

#[host_fn]
extern "ExtismHost" {
    fn env_var(key: &str) -> Option<String>; // FIXME--
}

#[plugin_fn]
pub fn _main(Json(input): Json<RealworldFetchPayload>) -> FnResult<Json<Value>> {
    let created_article_title = input.response.body["article"]
        .as_object()
        .and_then(|v| v["title"].as_str())
        .ok_or(Error::msg("article title not found"))?;
    let message = format!(
        "A new blog post titled '{}' was just created by yours truly!",
        created_article_title
    );

    let created_article_slug = input.response.body["article"]
        .as_object()
        .and_then(|v| v["slug"].as_str())
        .ok_or(Error::msg("article title not found"))?;
    let created_article_link = format!("http://localhost:5173/article/{}", created_article_slug);

    let req = HttpRequest::new("https://api.emailjs.com/api/v1.0/email/send-form")
        .with_method("POST")
        .with_header("Content-Type", "application/x-www-form-urlencoded");

    let service_id = unsafe { env_var("EMAILJS_SERVICE_ID")? }
        .ok_or(Error::msg("EMAILJS_SERVICE_ID not found"))?;
    let template_id = unsafe { env_var("EMAILJS_TEMPLATE_ID")? }
        .ok_or(Error::msg("EMAILJS_TEMPLATE_ID not found"))?;
    let user_id =
        unsafe { env_var("EMAILJS_USER_ID")? }.ok_or(Error::msg("EMAILJS_USER_ID not found"))?;
    let access_token = unsafe { env_var("EMAILJS_ACCESS_TOKEN")? }
        .ok_or(Error::msg("EMAILJS_ACCESS_TOKEN not found"))?;

    let params = [
        ("service_id", service_id.as_str()),
        ("template_id", template_id.as_str()),
        ("user_id", user_id.as_str()),
        ("accessToken", access_token.as_str()),
        ("to_name", "Alex Dunne"),
        ("from_name", "ZeroSync"),
        ("message", message.as_str()),
        ("to_email", "alex.dunne@zerosync.co"),
        ("link", created_article_link.as_str()),
    ];

    let encoded_params = serde_urlencoded::to_string(params)?;

    let res = http::request(&req, Some(encoded_params))?;

    Ok(Json(serde_json::json!({
        "status_code": res.status_code(),
        "body": String::from_utf8_lossy(res.body().as_slice())
    })))
}
