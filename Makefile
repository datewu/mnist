-include .envrc

.PHONY: build run 

# run cargo leptos watch in dev mode
build:
	#cargo leptos watch 
	rustup target add wasm32-unknown-unknown
	wasm-pack build --out-dir pkg --release --target web --no-typescript --no-default-features --features ndarray
run: build
	python3 -m http.server 8000

