use std::process::{exit, Stdio};

pub fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut imports = Default::default();
    let mut errors = 0;
    for arg in args {
        let data = std::fs::read(&arg).unwrap();

        imports = extism_pdk_test::check_imports(&data, Some(imports)).unwrap();
        let mut plugin = extism::PluginBuilder::new(
            extism::Manifest::new([extism::Wasm::data(data)]).with_allowed_host("extism.org"),
        )
        .with_wasi(true)
        .build()
        .unwrap();

        if plugin.function_exists("kitchen_sink") {
            println!("INFO Calling kitchen_sink using the Rust SDK");
            let output: Result<&str, extism::Error> = plugin.call("kitchen_sink", "test");
            match output {
                Ok(output) => {
                    if output != "test" {
                        println!("ERROR invalid results from kitchen_sink");
                        errors += 1
                    }
                }
                Err(e) => {
                    println!("ERROR kitchen_sink:");
                    println!("{:?}", e);
                    errors += 1;
                }
            }

            println!("INFO Calling kitchen_sink using the Go SDK/CLI");
            if std::process::Command::new("extism")
                .arg("call")
                .arg(&arg)
                .arg("kitchen_sink")
                .arg("--allow-host")
                .arg("extism.org")
                .arg("--wasi")
                .stdout(Stdio::null())
                .status()
                .is_err()
            {
                println!("ERROR unable to call kitchen_sink using extism CLI");
                errors += 1;
            }
        } else {
            println!("WARNING kitchen_sink function not found");
        }
    }

    println!("INFO Checking for Extism functions");
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
