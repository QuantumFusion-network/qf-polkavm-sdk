.PHONY: open-rustdoc rustdoc doc-all

open-rustdoc: doc-all
	cargo doc --open

rustdoc:
	rustdoc src/lib.rs -o target/doc

doc-all:
	./build_docs.sh
