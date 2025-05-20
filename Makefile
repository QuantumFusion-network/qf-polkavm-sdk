.PHONY: open-rustdoc rustdoc

open-rustdoc: rustdoc
	cargo doc --open

rustdoc:
	rustdoc src/lib.rs -o target/doc

