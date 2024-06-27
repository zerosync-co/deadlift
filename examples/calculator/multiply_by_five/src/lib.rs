use serde_json::Value;
use extism_pdk::*;

#[plugin_fn]
pub fn _main(input: Value) -> FnResult<Value> {
    let output = (input.as_i64().unwrap_or_default() * 5).into();
    Ok(output)
}
