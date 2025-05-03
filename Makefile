.PHONY: doc_gen
doc_gen:
	RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --lib --no-deps
