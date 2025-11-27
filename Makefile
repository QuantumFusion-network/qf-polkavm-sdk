.PHONY: open-rustdoc doc-all

open-rustdoc: doc-all
	cargo doc --open

doc-all:
	./build_docs.sh
