package main

import (
	"github.com/extism/go-pdk"
	_ "github.com/extism/go-pdk/wasi-reactor"
)

//export kitchen_sink
func kitchen_sink() {
	input := pdk.InputString()

	// Config
	pdk.GetConfig("test")

	// // Var
	pdk.SetVar("test_var", []byte("something"))
	v := pdk.GetVar("test_var")
	if string(v) != "something" {
		pdk.SetErrorString("Invalid var")
		panic("Invalid var")
	}

	// HTTP
	req := pdk.NewHTTPRequest(pdk.MethodGet, "https://extism.org")
	res := req.Send()
	if res.Status() != 200 {
		pdk.SetErrorString("Invalid HTTP response")
		panic("Invalid HTTP response")
	}
	mem := res.Memory()
	mem.Free()

	// Logging
	pdk.Log(pdk.LogInfo, "INFO")
	pdk.Log(pdk.LogDebug, "DEBUG")
	pdk.Log(pdk.LogWarn, "WARN")
	pdk.Log(pdk.LogError, "ERROR")

	pdk.OutputString(input)
}

func main() {}
