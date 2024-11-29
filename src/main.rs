use extism::{convert::Json, FromBytes, Manifest, PluginBuilder, ToBytes, Wasm};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromBytes, ToBytes)]
#[encoding(Json)]
struct ReportData {
    obj: serde_json::Value,
}

fn main() {
    test("jsplugin/jsplugin.wasm", 4000, 5000);
    test(
        "rustplugin/target/wasm32-unknown-unknown/debug/rustplugin.wasm",
        4000,
        5000,
    );
}

fn test(wasm_file: &str, max_kb: usize, key_value_per_test: u32) {
    let manifest = Manifest::new([Wasm::file(wasm_file)]);

    let mut plugin = PluginBuilder::new(manifest)
        .with_wasi(true)
        .build()
        .unwrap();

    let mut data: serde_json::Value = serde_json::Value::Object(Map::new());

    loop {
        let as_object = data.as_object_mut().unwrap();
        for _ in 0..key_value_per_test {
            let key = Uuid::new_v4().to_string();
            let value = Uuid::new_v4().to_string();
            as_object.insert(key, serde_json::Value::String(value));
        }

        let kbs = serde_json::to_string(&data).unwrap().len() / 1024;
        if kbs > max_kb {
            return println!("{wasm_file} finished");
        }

        println!("trying {kbs}kbs",);

        data = match plugin.call("check", &data) {
            Ok(d) => d,
            Err(e) => {
                return println!("{wasm_file} failed with error {e:?}");
            }
        }
    }
}
