pub fn read_file(path: &std::path::Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", path.display()))
}

pub fn write_file(path: &std::path::Path, contents: &str) {
    std::fs::write(path, contents)
        .unwrap_or_else(|_| panic!("Failed to write to file: {}", path.display()));
}

pub fn read_dir(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", path.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect()
}
