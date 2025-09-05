.PHONY: lint format

build:
	$(format)
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

id:
	maelstrom/maelstrom test -w unique-ids --bin bin/debug/challenge \
		--time-limit 30 \
		--rate 1000 \
		--node-count 3 \
		--availability total \
		--nemesis partition