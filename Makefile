.PHONY: open-rustdoc doc-all

open-rustdoc: doc-all
	cargo doc -p qf-polkavm-sdk --open

doc-all:
	./build_docs.sh
