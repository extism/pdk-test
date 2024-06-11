NAME=haskell-pdk-template

build:
	wasm32-wasi-cabal build
	cp `find dist-newstyle/build/wasm32-wasi/ -name kitchen-sink.wasm` kitchen_sink.wasm

test:
	extism call plugin.wasm greet --wasi --input "Hello!" --allow-host 'extism.org'
