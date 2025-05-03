@docgen *args='':
  RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --lib --no-deps {{args}}

@prepare type number:
  cargo generate --path template --name {{type}}_{{number}}
  git restore Cargo.toml
