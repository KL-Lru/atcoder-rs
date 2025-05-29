@docgen *args='': clean build generate_index
  RUSTDOCFLAGS="--default-theme ayu --extend-css custom/extend.css --enable-index-page --index-page custom/index.md --markdown-css top.css --markdown-no-toc --html-in-header custom/katex.html -Zunstable-options" cargo +nightly doc --lib --no-deps {{args}}
  cp custom/top.css target/doc/

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

@build:
  RUSTFLAGS="-Awarnings" cargo build --quiet

@generate_index:
  cargo run --bin indexgen
