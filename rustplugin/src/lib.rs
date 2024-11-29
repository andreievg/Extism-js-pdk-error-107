use extism_pdk::*;

#[plugin_fn]
pub fn check(name: String) -> FnResult<String> {
    let sup: serde_json::Value = serde_json::from_str(&name).unwrap();
    Ok(serde_json::to_string(&sup).unwrap())
}
