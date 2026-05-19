use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
use std::io::ErrorKind;
use uuid::Uuid;

pub const MANIFEST_VERSION: u8 = 1;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AssetType {
    ROOT,
    META,
    AUDIO,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AssetRecord {
    #[serde(with = "uuid::serde::simple")]
    id: Uuid,
    filename: String,
    checksum: String,
    path: Vec<String>,
}

impl AssetRecord {
    pub fn new(checksum: String, filename: String, path: &Vec<String>) -> AssetRecord {
        AssetRecord {
            checksum,
            filename,
            path: path.clone(),
            id: Uuid::now_v7(),
        }
    }
    pub async fn from_file(
        bytes: &[u8],
        filename: &str,
        path: &Vec<String>,
    ) -> std::io::Result<AssetRecord> {
        let mut hash = Sha256::new();
        if !bytes.is_empty() {
            hash.update(&bytes);
        } else {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "file is empty",
            ));
        }
        Ok(Self::new(
            const_hex::encode(hash.finalize()),
            String::from(filename),
            &path,
        ))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ManifestRecord {
    version: u8,
    assets: Vec<AssetRecord>,
}

impl ManifestRecord {
    pub fn new() -> Self {
        Self {
            version: MANIFEST_VERSION,
            assets: Vec::new(),
        }
    }

    pub async fn add_asset(
        &mut self,
        file: &[u8],
        filename: &str,
        path: &Vec<String>,
    ) -> std::io::Result<Uuid> {
        let record = AssetRecord::from_file(file, filename, path).await?;
        let res = record.id;
        self.assets.push(record);
        Ok(res)
    }

    pub fn add_asset_record(&mut self, record: AssetRecord) -> &mut Self {
        self.assets.push(record);
        self
    }
}

impl Default for ManifestRecord {
    fn default() -> Self {
        Self::new()
    }
}
