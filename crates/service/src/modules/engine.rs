use extism::*;

type Env = std::collections::HashMap<String, String>;

// FIXME--
host_fn!(env_var(env: Env; key: &str) -> Option<String> {
    let env = env.get()?;
    let env = match env.lock() {
        Ok(v) => v,
        Err(_) => return Ok(None)
    };
    Ok(env.get(key).cloned())
});

pub fn get_plugin_from_data(data: Vec<u8>) -> Plugin {
    let env = UserData::new(std::env::vars().collect::<Env>());

    let manifest = Manifest::new([Wasm::Data {
        data,
        meta: WasmMetadata::default(),
    }])
    .with_allowed_host("api.emailjs.com");

    PluginBuilder::new(manifest)
        .with_wasi(true)
        .with_function("env_var", [PTR], [PTR], env.clone(), env_var)
        .build()
        .unwrap() // FIXME--
}
