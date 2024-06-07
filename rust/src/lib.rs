#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn kitchen_sink() -> FnResult<()> {
    let input = input::<String>()?;

    // Config
    let x = config::get("test");

    // Var
    var::set("test_var", "something")?;
    let var0 = var::get::<String>("test_var")?;
    assert!(var0.is_some());
    assert!(var0.unwrap() == "something");

    // HTTP
    let req = extism_manifest::HttpRequest::new("https://extism.org");
    let res = http::request::<String>(&req, None)?;
    assert!(res.status_code() == 200);
    res.into_memory().free();

    // Logging
    info!("INFO");
    debug!("DEBUG");
    warn!("WARN");
    error!("ERROR");

    // Output
    output(input)?;

    Ok(())
}
