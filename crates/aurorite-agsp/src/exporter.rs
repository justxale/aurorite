use crate::common::ManifestRecord;
use async_compression::Level;
use async_compression::tokio::write::ZstdEncoder;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::{DirEntry, File};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use tokio_tar::{Builder, Header};

async fn add_file(
    tar: &mut Builder<ZstdEncoder<impl AsyncWrite + Send + Unpin>>,
    manifest: &mut ManifestRecord,
    parents: &Vec<String>,
    entry: &DirEntry,
) -> Result<(), std::io::Error> {
    tracing::debug!("file found: {}", entry.path().display());
    let mut bytes = Vec::new();
    let mut reader = File::open(entry.path()).await?;
    reader.read_to_end(&mut bytes).await?;
    let id = manifest
        .add_asset(&bytes, entry.file_name().to_str().unwrap(), parents)
        .await?;
    reader.seek(SeekFrom::Start(0)).await?;
    tar.append_file(id.simple().to_string(), &mut reader).await
}

async fn read_dir_files(
    tar: &mut Builder<ZstdEncoder<impl AsyncWrite + Send + Unpin>>,
    manifest: &mut ManifestRecord,
    dir_path: &Path,
    parent_path: &[String],
) -> Result<(), std::io::Error> {
    let mut parents = Vec::from(parent_path);
    if dir_path.is_dir()
        && let Some(dirname) = dir_path.file_name()
    {
        parents.push(dirname.to_string_lossy().to_string());
    }
    let mut dir = tokio::fs::read_dir(dir_path).await?;
    while let Some(entry) = dir.next_entry().await? {
        if let Ok(entry_type) = entry.file_type().await {
            if entry_type.is_file() {
                add_file(tar, manifest, &parents, &entry).await?;
            } else if entry_type.is_symlink() {
                tracing::warn!("symlink found: {}", entry.path().display());
            } else if entry_type.is_dir() {
                tracing::debug!("directory found: {}", entry.path().display());
                Box::pin(read_dir_files(tar, manifest, &entry.path(), &parents)).await?;
            }
        }
    }
    Ok(())
}

pub async fn export(root_dir: PathBuf) -> impl AsyncRead + Unpin {
    tracing::debug!("started exporting assets from {}", root_dir.display());
    let parents = Vec::new();
    let (reader, writer) = tokio::io::duplex(256 * 1024);

    tokio::spawn(async move {
        let encoder = ZstdEncoder::with_quality(writer, Level::Best);
        let mut manifest = ManifestRecord::new();
        let mut tar = Builder::new(encoder);
        let mut dir = tokio::fs::read_dir(&root_dir).await?;

        while let Ok(Some(entry)) = dir.next_entry().await {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    add_file(&mut tar, &mut manifest, &parents, &entry).await?;
                } else if file_type.is_dir() {
                    read_dir_files(&mut tar, &mut manifest, &entry.path(), &parents).await?;
                }
            }
        }
        tracing::debug!("writing manifest");
        let manifest_bytes = toml::to_string(&manifest).unwrap();
        let mut header = Header::new_gnu();
        header.set_path("MANIFEST")?;
        header.set_size(manifest_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append(&header, manifest_bytes.as_bytes()).await?;
        tracing::info!("successfully exported AGSP");
        let mut encoder = tar.into_inner().await?;
        encoder.shutdown().await?;
        Ok::<(), std::io::Error>(())
    });

    reader
}
