build:
	cargo build --release
	mkdir -p bin
	cp target/release/pmt bin/pmt

clean:
	cargo clean

release: clean test build

TESTS ?= ""
test:
	cargo test $(TESTS)
