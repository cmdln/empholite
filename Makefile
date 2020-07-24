docker_tag=0.2.4

all: client/pkg/server_bg.wasm client/pkg/bundle.js

clean:
	rm -rf client/pkg/*
	cargo clean

client/pkg/client_bg.wasm: client/main.js \
	client/src/components/*.rs \
	client/src/components/alert/*.rs \
	client/src/components/editor/*.rs \
	client/src/components/editor/rule_editor/*.rs \
	client/src/components/home/*.rs \
	client/src/types/*.rs \
	client/src/lib.rs \
	shared/src/*.rs
	mkdir -p client/pkg
	wasm-pack build --no-typescript --dev -t web client

client/pkg/bundle.js: client/pkg/client_bg.wasm
	rollup client/main.js --format iife --file client/pkg/bundle.js

client-watch:
	while true; do \
		make --silent client/pkg/bundle.js; \
		sleep 1; \
	done

docker:
	wasm-pack build --no-typescript --release -t web client
	rollup client/main.js --format iife --file client/pkg/bundle.js
	cargo build -p empholite --release
	docker build -t cmdln/empholite:latest -t cmdln/empholite:$(docker_tag) .

.PHONY: all clean client-watch docker
