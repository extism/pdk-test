use std::process::exit;

pub fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut imports = Default::default();
    for arg in args {
        let data = std::fs::read(&arg).unwrap();
        imports = extism_pdk_test::check_imports(&data, Some(imports)).unwrap();
    }
    let mut errors = 0;

    for found in &imports.found {
        println!("FOUND extism:host/env::{}", found);
    }

    for missing in &imports.missing {
        errors += 1;
        println!("MISSING extism:host/env::{}", missing);
    }

    for unknown in &imports.unknown {
        errors += 1;
        println!("UNKNOWN extism:host/env::{}", unknown);
    }

    println!("-----");
    println!("Extism function coverage: {:.1}%", imports.percent_found());

    exit(errors)
}
