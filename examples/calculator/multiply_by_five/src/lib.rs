use serde_json::Value;
use deadlift_util::modulify;

#[modulify]
fn main(input: Value) -> Value {
    println!("module: multiply_by_five; received input: {}", input);
    let output = (input.as_i64().unwrap_or_default() * 5).into();
    println!("module: multiply_by_five; output: {}", output);
    output
}
