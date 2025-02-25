use wasmtime::{Config, Engine, Module};

const WASM: &[u8] = include_bytes!("../rust_wasm_module.wasm");

pub fn create_module() -> Module {
    let mut config = Config::new();
    config.native_unwind_info(false);
    let engine = Engine::new(&config).expect("failed to create engine");
    Module::new(&engine, WASM).expect("failed to create module")
}
