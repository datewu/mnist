-include .envrc

.PHONY: watch release test test-release fmt-leptos prepare

# run cargo leptos watch in dev mode
watch:
	#cargo leptos watch 
	cargo leptos watch --hot-reload --features ndarray
release:
	cargo leptos build --release

test:
	cargo leptos end-to-end

test-release: test
	cargo leptos end-to-end --release
	

fmt-leptos:
	leptosfmt ./**/*.rs

prepare:
	sqlx migrate run
	cargo sqlx prepare --all --  --features ssr

chinese:
	cargo run --features ssr --bin english_chinese --release
