use std::path::PathBuf;

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
}

pub fn doc_dir() -> PathBuf {
    manifest_dir().join("docs")
}

pub fn doc_file(binary: &str) -> PathBuf {
    doc_dir().join(format!("{binary}.md"))
}

pub fn src_dir() -> PathBuf {
    manifest_dir().join("src")
}

pub fn lib_file() -> PathBuf {
    src_dir().join("lib.rs")
}

pub fn binary_dir() -> PathBuf {
    src_dir().join("bin")
}

pub fn binary_file(binary: &str) -> PathBuf {
    binary_dir().join(format!("{binary}.rs"))
}

pub fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"))
}

pub fn module_file() -> PathBuf {
    out_dir().join("modules.rs")
}

pub fn out_binary_file(binary: &str) -> PathBuf {
    out_dir().join(format!("{binary}.rs"))
}
