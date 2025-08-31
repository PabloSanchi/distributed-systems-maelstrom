.PHONY: lint format

build:
	cargo build --target-dir bin/

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

format:
	cargo fmt --all

clean:
	rm -rf bin/
	rm -rf target/
	rm -rf store/

echo:
	maelstrom/maelstrom test -w echo --bin bin/debug/challenge \
		--node-count 1 \
		--time-limit 10
