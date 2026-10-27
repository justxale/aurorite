use crate::checksum::compute_hash;
use crate::{AssetRecord, MAX_PACKAGE_SIZE, ManifestRecord};
use async_compression::tokio::bufread::ZstdDecoder;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use tokio::task::JoinSet;
use tokio_tar::Archive;
use tokio_util::io::ReaderStream;
use crate::consts::{package_path, PACKAGE_CACHE_FOLDER};

#[derive(Debug)]
pub enum AgspError {
    IoFailure(String),
    InvalidManifest,
    InvalidChecksum,
}

impl Display for AgspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Error for AgspError {}

impl From<std::io::Error> for AgspError {
    fn from(err: std::io::Error) -> Self {
        AgspError::IoFailure(format!("{}", err))
    }
}

async fn inspect_manifest(mut reader: impl AsyncRead + Unpin) -> Result<ManifestRecord, AgspError> {
    let mut buf = Vec::<u8>::new();
    reader
        .read_to_end(&mut buf)
        .await
        .map_err(AgspError::from)?;
    toml::from_slice::<ManifestRecord>(&buf).map_err(|_| AgspError::InvalidManifest)
}

async fn import_asset(
    record: Arc<AssetRecord>,
    package_name: Arc<String>,
) -> Result<(), AgspError> {
    let path: PathBuf = PACKAGE_CACHE_FOLDER.join(record.id.as_simple().to_string());
    let hash = compute_hash(ReaderStream::new(File::open(&path).await?)).await?;
    if record.checksum != hash {
        return Err(AgspError::InvalidChecksum);
    }
    tracing::debug!("{} checksum passed", record.id);
    let mut output_path = package_path(&package_name);
    output_path.extend(&record.path);
    tokio::fs::create_dir_all(&output_path).await?;
    output_path.push(&record.filename);
    tokio::fs::rename(path, output_path).await?;
    Ok(())
}

pub async fn import(
    reader: impl AsyncRead + Unpin,
) -> Result<Vec<Result<(), AgspError>>, AgspError> {
    let decoder = ZstdDecoder::new(BufReader::new(reader));
    let mut tar = Archive::new(decoder.take(MAX_PACKAGE_SIZE as u64));
    tar.unpack(&*PACKAGE_CACHE_FOLDER).await?;
    let manifest = inspect_manifest(File::open(&PACKAGE_CACHE_FOLDER.join("MANIFEST")).await?).await?;
    let (chunks, rem) = manifest.assets.as_chunks::<10>();
    for chunk in chunks {
        let mut set = JoinSet::<Result<(), AgspError>>::new();
        for record in chunk {
            set.spawn(import_asset(record.clone(), manifest.package_name.clone()));
        }
        let _ = set.join_all().await;
    }
    let mut set = JoinSet::<Result<(), AgspError>>::new();
    for record in rem.iter() {
        set.spawn(import_asset(record.clone(), manifest.package_name.clone()));
    }
    let results = set.join_all().await;
    tokio::fs::remove_dir_all(&*PACKAGE_CACHE_FOLDER).await?;
    Ok(results)
}
