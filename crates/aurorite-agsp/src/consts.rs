use std::iter::{IntoIterator, Iterator};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::task::JoinSet;
use uuid::Uuid;
use aurorite_util::env;

pub static PACKAGE_CACHE_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    [".", "cache", "extract"].into_iter().collect()  
});
pub static PACKAGE_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| env().data_root.join([".", "packages"].into_iter().collect::<PathBuf>()));

pub static TRACK_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    env().data_root.join([".", "tracks"].into_iter().collect::<PathBuf>())
});

pub async fn create_paths() {
    tracing::info!("creating assets directories");
    let mut set = JoinSet::new();
    for path in [&TRACK_FOLDER, &PACKAGE_FOLDER] {
        set.spawn(tokio::fs::create_dir_all(&**path));
    }
    set.join_all().await;
}

pub fn extraction_path(id: Uuid) -> PathBuf {
    PACKAGE_CACHE_FOLDER.join(id.simple().to_string())
}

pub fn package_path(package_name: &str) -> PathBuf {
    PACKAGE_FOLDER.join(package_name)
}

pub fn track_path(id: Uuid) -> PathBuf {
    TRACK_FOLDER.join(id.simple().to_string())
}