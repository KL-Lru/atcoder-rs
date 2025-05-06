use crate::io;
use crate::path;

use crate::file::{file_stem, is_rs_file};

pub fn binary_list() -> Vec<String> {
    io::read_dir(&path::binary_dir())
        .iter()
        .filter_map(|entry| match is_rs_file(entry) {
            true => Some(file_stem(entry)),
            false => None,
        })
        .collect()
}

pub fn generate_lib_rs() {
    let lib_file = path::lib_file();
    let contents = [no_source_attributes(), readme_attributes(), module_magic()];

    io::write_file(&lib_file, &contents.join("\n"));
}

pub fn generate_module_rs() {
    let module_file = path::module_file();
    let contents = binary_list()
        .iter()
        .map(|binary| format!("pub mod {binary};"))
        .collect::<Vec<_>>()
        .join("\n");

    io::write_file(&module_file, &contents);
}

pub fn generate_binary_module_rs(binary: &str) {
    let binary_file = path::binary_file(binary);
    if !binary_file.exists() {
        return;
    }

    let contents = document_attributes(binary);

    io::write_file(&path::out_binary_file(binary), &contents);
}

fn document_attributes(binary: &str) -> String {
    [
        markdown_attributes(binary),
        cfg_doc_attribute("\"```rust,no_run\""),
        source_code_attributes(binary),
        cfg_doc_attribute("\"```\""),
    ]
    .join("\n")
}

fn module_magic() -> String {
    r#"include!(concat!(env!("OUT_DIR"), "/modules.rs"));"#.to_string()
}

fn no_source_attributes() -> String {
    r#"#![doc(html_no_source)]"#.to_string()
}

fn cfg_doc_attribute(doc: &str) -> String {
    format!("#![cfg_attr(doc, doc = {doc})]")
}

fn readme_attributes() -> String {
    let readme_file = path::manifest_dir().join("README.md");

    if !readme_file.exists() {
        return String::new();
    }

    cfg_doc_attribute(&format!("include_str!(\"{}\")", readme_file.display()))
}

fn markdown_attributes(binary: &str) -> String {
    let doc_file = path::doc_file(binary);

    if !doc_file.exists() {
        return String::new();
    }

    cfg_doc_attribute(&format!("include_str!(\"{}\")", doc_file.display()))
}

fn source_code_attributes(binary: &str) -> String {
    let binary_file = path::binary_file(binary);

    if !binary_file.exists() {
        return String::new();
    }

    cfg_doc_attribute(&format!("include_str!(\"{}\")", binary_file.display()))
}
