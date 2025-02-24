use wasmtime::{Engine, Module};

const WASM: &[u8] = include_bytes!("../rust_wasm_module.wasm");

pub fn create_module() -> Module {
    let engine = Engine::default();
    Module::new(&engine, WASM).expect("failed to create module")
}
