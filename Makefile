run:
	cargo build --package is_odd_lib
	cp target/debug/libis_odd_lib.* ./
	cargo run --package rust-dylib-example
clean:
	cargo clean
	rm libis_odd_lib.*