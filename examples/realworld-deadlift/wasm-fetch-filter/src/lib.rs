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

#[plugin_fn]
pub fn _main(
    Json(input): Json<RealworldFetchPayload>,
) -> FnResult<Json<Option<RealworldFetchPayload>>> {
    let is_post_request = input.request.opts["method"]
        .as_str()
        .map_or(false, |v| v == "POST");
    let is_create_article_url = input.request.url == "https://api.realworld.io/api/articles";

    if is_post_request && is_create_article_url {
        Ok(Json(Some(input)))
    } else {
        Ok(Json(None))
    }
}
