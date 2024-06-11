const std = @import("std");
const extism_pdk = @import("extism-pdk");
const Plugin = extism_pdk.Plugin;

const allocator = std.heap.wasm_allocator;

export fn kitchen_sink() i32 {
    const plugin = Plugin.init(allocator);
    const input = plugin.getInput() catch unreachable;
    defer allocator.free(input);

    _ = plugin.getConfig("test") catch unreachable;

    plugin.setVar("test_var", "something");

    const c = plugin.getVar("test_var") catch unreachable orelse "";
    if (!std.mem.eql(u8, c, "something")) {
        unreachable;
    }

    // HTTP
    var req = extism_pdk.http.HttpRequest.init("GET", "https://extism.org");
    defer req.deinit(allocator);
    const res = plugin.request(req, null) catch unreachable;
    defer res.deinit();

    if (res.status != 200) {
        plugin.setError("request failed");
        return @as(i32, res.status);
    }

    // get the bytes for the res body
    const body = res.body(allocator) catch unreachable;
    defer allocator.free(body);

    // Logging
    plugin.log(.Info, "INFO");
    plugin.log(.Debug, "DEBUG");
    plugin.log(.Warn, "WARN");
    plugin.log(.Error, "ERROR");

    plugin.output(input);
    return 0;
}
