@docgen *args='': clean
  RUSTFLAGS="-Awarnings" cargo build --quiet
  RUSTDOCFLAGS="--default-theme ayu --enable-index-page -Zunstable-options" cargo +nightly doc --lib --no-deps {{args}}

@docserve:
  npx browser-sync start --server "target/doc/" --files "target/doc/" --open --watch

@prepare type number:
  cargo generate --path template --name {{type}}_{{number}}
  git restore Cargo.toml

@clean:
  cargo clean
  find . -name lib.rs | grep -v lib/ | xargs --no-run-if-empty rm
