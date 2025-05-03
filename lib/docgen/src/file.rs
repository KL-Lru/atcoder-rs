pub fn is_rs_file(path: &std::path::Path) -> bool {
    path.is_file() && has_extension(path, "rs")
}

pub fn is_md_file(path: &std::path::Path) -> bool {
    path.is_file() && has_extension(path, "rs")
}

fn has_extension(path: &std::path::Path, ext: &str) -> bool {
    path.extension().is_some_and(|extension| extension == ext)
}

pub fn file_stem(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}
