use serde_json::Value;
use deadlift_util::modulify;

#[modulify]
fn main(input: Value) -> Value {
    (input.as_i64().unwrap_or_default() * 5).into()
}
