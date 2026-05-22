use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const MANIFEST_VERSION: u8 = 1;
pub const MAX_PACKAGE_SIZE: usize = 8 * 1024 * 1024 * 1024; // 8 GiB

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AssetType {
    ROOT,
    META,
    AUDIO,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AssetRecord {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub filename: String,
    pub checksum: String,
    pub path: Vec<String>,
}

impl AssetRecord {
    pub fn new(checksum: String, filename: &str, path: &[String]) -> AssetRecord {
        AssetRecord {
            checksum,
            filename: String::from(filename),
            path: path.into(),
            id: Uuid::now_v7(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ManifestRecord {
    pub package_name: Arc<String>,
    pub version: u8,
    pub assets: Vec<Arc<AssetRecord>>,
}

impl ManifestRecord {
    pub fn new() -> Self {
        Self {
            package_name: Arc::new(String::from("example-package")),
            version: MANIFEST_VERSION,
            assets: Vec::new(),
        }
    }

    pub fn add_asset(&mut self, record: AssetRecord) -> uuid::Uuid {
        let id = record.id;
        self.assets.push(Arc::new(record));
        id
    }
}

impl Default for ManifestRecord {
    fn default() -> Self {
        Self::new()
    }
}
