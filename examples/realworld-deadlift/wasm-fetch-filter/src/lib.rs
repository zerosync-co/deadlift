use serde::{Deserialize, Serialize};
use serde_json::Value;
use deadlift_util::modulify;

#[derive(Debug, Deserialize, Serialize)]
struct RealworldFetchPayload {
    request: RequestInfo,
    response: ResponseInfo
}

#[derive(Debug, Deserialize, Serialize)]
struct RequestInfo {
    opts: Value,
    url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseInfo {
    body: Value,
    url: String
}


#[modulify]
fn main(input: RealworldFetchPayload) -> Option<RealworldFetchPayload> {
    println!("module: received realworld fetch payload; {:?}", input);

    let is_post_request = input.request.opts["method"].as_str().map_or(false, |v| v == "POST");
    let is_create_article_url = input.request.url == "https://api.realworld.io/api/articles";

    if is_post_request && is_create_article_url {
        Some(input)
    } else {
        None
    }
}
