all: client/pkg/server_bg.wasm client/pkg/bundle.js

clean:
	rm -rf client/pkg/*
	cargo clean

client/pkg/client_bg.wasm: client/main.js \
	client/src/lib.rs
	mkdir -p client/pkg
	wasm-pack build --no-typescript --dev -t web client

client/pkg/bundle.js: client/pkg/client_bg.wasm
	rollup client/main.js --format iife --file client/pkg/bundle.js

client-watch:
	while true; do \
		make --silent client/pkg/bundle.js; \
		sleep 1; \
	done

.PHONY: all clean client-watch
