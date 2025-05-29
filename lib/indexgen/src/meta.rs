use std::path::PathBuf;

use cargo_metadata::{Metadata, MetadataCommand, Package};

fn metadata() -> Metadata {
    MetadataCommand::new().exec().expect("Failed to get meta")
}

pub fn workspace_members() -> Vec<Package> {
    metadata()
        .workspace_packages()
        .iter()
        .map(|&p| p.clone())
        .collect()
}

pub fn workspace_root() -> PathBuf {
    metadata().workspace_root.into_std_path_buf()
}
