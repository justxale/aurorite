use std::iter::{IntoIterator, Iterator};
use std::path::PathBuf;
use std::sync::LazyLock;
use uuid::Uuid;
use aurorite_util::env;

pub static PACKAGE_CACHE_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    [".", "cache", "extract"].into_iter().collect()  
});
pub static PACKAGE_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| [".", "packages"].into_iter().collect());

pub static TRACK_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    [".", "tracks"].into_iter().collect()
});

pub fn extraction_path(id: Uuid) -> PathBuf {
    env().data_root.join(&*PACKAGE_CACHE_FOLDER).join(id.simple().to_string())
}

pub fn package_path(package_name: &str) -> PathBuf {
    env().data_root.join(&*PACKAGE_FOLDER).join(package_name)
}

pub fn track_path(id: Uuid) -> PathBuf {
    env().data_root.join(&*TRACK_FOLDER).join(id.simple().to_string())
}