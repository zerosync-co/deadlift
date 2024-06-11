use serde_json::Value;
use deadlift_util::modulify;

#[modulify]
fn main(input: Value) -> Value {
    println!("module: add_ten; received input: {}", input);
    let output = (input.as_i64().unwrap_or_default() + 10).into();
    println!("module: add_ten; output: {}", output);
    output
}
