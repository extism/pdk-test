use std::collections::BTreeSet;

use extism::Error;

pub struct Imports {
    pub missing: BTreeSet<String>,
    pub found: BTreeSet<String>,
    pub unknown: BTreeSet<String>,
}

impl Default for Imports {
    fn default() -> Self {
        Imports {
            missing: EXTISM_ENV_FUNCS.iter().map(|x| x.to_string()).collect(),
            found: Default::default(),
            unknown: Default::default(),
        }
    }
}

impl Imports {
    pub fn percent_found(&self) -> f32 {
        let found = self.found.len() as f32;
        let total = EXTISM_ENV_FUNCS.len() as f32;
        (found / total) * 100.0
    }
}

const EXTISM_ENV_FUNCS: &'static [&'static str] = &[
    "alloc",
    "length",
    "length_unsafe",
    "free",
    "error_set",
    "load_u64",
    "load_u8",
    "input_load_u64",
    "input_load_u8",
    "input_length",
    "store_u64",
    "store_u8",
    "output_set",
    "config_get",
    "var_set",
    "var_get",
    "http_request",
    "http_status_code",
    "log_info",
    "log_error",
    "log_debug",
    "log_warn",
];

pub fn check_imports(data: &[u8], report: Option<Imports>) -> Result<Imports, Error> {
    let mut config = wasmtime::Config::new();
    config
        .epoch_interruption(true)
        .wasm_tail_call(true)
        .wasm_function_references(true);
    // .wasm_gc(true);
    let engine = wasmtime::Engine::new(&config)?;
    let module = wasmtime::Module::new(&engine, data)?;

    let imports = module.imports();

    let mut report = report.unwrap_or_default();

    for import in imports {
        if import.module() == "extism:host/env" {
            if !EXTISM_ENV_FUNCS.contains(&import.name()) {
                report.unknown.insert(import.name().to_string());
            } else {
                report.found.insert(import.name().to_string());
                report.missing.remove(import.name());
            }
        }
    }

    Ok(report)
}
