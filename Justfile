@docgen *args='': clean
  RUSTFLAGS="-Awarnings" cargo build --quiet
  RUSTDOCFLAGS="--default-theme ayu --enable-index-page --html-in-header custom/katex.html -Zunstable-options" cargo +nightly doc --lib --no-deps {{args}}

@docserve:
  npx browser-sync start --server "target/doc/" --files "target/doc/" --open --watch

@prepare type number:
  cargo generate --path template --name {{type}}_{{number}}
  git restore Cargo.toml
  git add -f {{type}}_{{number}}/Cargo.toml
  git add -f {{type}}_{{number}}/README.md
  git commit -m "add $(echo {{type}} | tr [a-z] [A-Z]) {{number}}"

@solve problem:
  git add -f {{invocation_directory()}}/docs/{{problem}}.md
  git add -f {{invocation_directory()}}/src/bin/{{problem}}.rs
  git commit -m "solve $(basename {{invocation_directory()}} | tr [a-z] [A-Z] | sed -e 's/_/ /g') $(echo {{problem}} | tr [a-z] [A-Z])"

@clean:
  cargo clean
  find . -name lib.rs | grep -v lib/ | xargs --no-run-if-empty rm
