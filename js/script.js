function kitchen_sink() {
  const input = Host.inputString();

  // Config
  Config.get("test");

  // Vars
  Var.set("test_var", "something");
  const testVar = Var.getString("test_var");
  if (testVar !== "something") {
    throw "Invalid test_var";
  }

  // HTTP
  let resp = Http.request({
    url: "https://extism.org",
    method: "GET",
  }, null);
  if (resp.status !== 200) {
    throw "Invalid HTTP status code";
  }

  // Logging
  console.log("INFO");
  console.debug("DEBUG");
  console.warn("WARN");
  console.error("ERROR");

  Host.outputString(input);
  return 0;
}

module.exports = { kitchen_sink };
