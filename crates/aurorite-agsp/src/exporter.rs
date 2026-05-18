use std::io::SeekFrom;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use tokio_tar::{Builder, Header};
use async_compression::tokio::write::GzipEncoder;
use crate::common::{AssetType, ManifestRecord};

async fn read_dir_files(
    tar: &mut Builder<GzipEncoder<Vec<u8>>>,
    manifest: &mut ManifestRecord,
    dir_path: &Path,
    dir_type: AssetType
) -> Result<(), std::io::Error> {
    let mut dir = tokio::fs::read_dir(dir_path).await?;
    while let Ok(Some(file)) = dir.next_entry().await {
        if let Ok(file_type) = file.file_type().await && file_type.is_file() {
            let mut bytes = Vec::new();
            let mut reader = File::open(&file.path()).await?;
            reader.read_to_end(&mut bytes).await?;
            let id = manifest.add_asset(
                &bytes,
                file.file_name().to_str().unwrap(),
                dir_type.clone()
            ).await?;
            reader.seek(SeekFrom::Start(0)).await?;
            tar.append_file(id.simple().to_string(), &mut reader).await?;
        }
    };
    Ok(())
}

pub async fn export(root_dir: &Path) -> std::io::Result<Vec<u8>> {
    let encoder = GzipEncoder::new(Vec::new());
    let mut manifest = ManifestRecord::new();
    let mut tar = Builder::new(encoder);
    let mut dir = tokio::fs::read_dir(root_dir).await?;

    while let Ok(Some(file)) = dir.next_entry().await {
        if let Ok(file_type) = file.file_type().await {
            if file_type.is_file() {
                let mut bytes = Vec::new();
                let mut reader = File::open(&file.path()).await?;
                reader.read_to_end(&mut bytes).await?;
                manifest.add_asset(&bytes, file.file_name().to_str().unwrap(), AssetType::ROOT).await?;
            } else if file_type.is_dir() {
                match file.file_name().to_str() {
                    Some("meta") => {
                        read_dir_files(&mut tar, &mut manifest, &file.path(), AssetType::META).await?
                    }
                    Some("audio") => {
                        read_dir_files(&mut tar, &mut manifest, &file.path(), AssetType::AUDIO).await?
                    }
                    Some(_) => {}
                    None => {}
                }
            }
        }
    };
    let manifest_bytes = toml::to_string(&manifest).unwrap();
    let mut header = Header::new_gnu();
    header.set_path("MANIFEST")?;
    header.set_size(manifest_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append(&header, manifest_bytes.as_bytes()).await?;
    Ok(tar.into_inner().await?.into_inner())
}

pub async fn export_to_file(root_dir: &Path, file: &mut (impl AsyncWrite + Unpin)) -> std::io::Result<()> {
    file.write_all(&export(root_dir).await?).await
}