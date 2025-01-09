-include .envrc

.PHONY: watch release serve test-release fmt-leptos

# run cargo leptos watch in dev mode
watch:
	#cargo leptos watch 
	cargo leptos watch --hot-reload  --features ndarray
release:
	cargo leptos build --release --features ndarray

serve: release
	./target/release/mnist

test-release: test
	cargo leptos end-to-end --release

fmt-leptos:
	leptosfmt ./**/*.rs

