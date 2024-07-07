build:
	cargo build --release
	mkdir -p bin
	cp target/release/pmt bin/pmt

clean:
	cargo clean

format:
	cargo fmt

release: clean format build

TESTS ?= ""
test: format
	cargo test $(TESTS)
