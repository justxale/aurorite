use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};
use async_compression::tokio::bufread::ZstdDecoder;
use tokio_tar::Archive;
use crate::{ManifestRecord, MAX_PACKAGE_SIZE};

#[derive(Debug)]
enum AgspError {
    IoFailure,
    InvalidManifest
}

impl Display for AgspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", &self))
    }
}

impl Error for AgspError {}

async fn inspect_manifest(mut reader: impl AsyncRead + Unpin) -> Result<ManifestRecord, AgspError> {
    let mut buf = Vec::<u8>::new();
    reader.read_to_end(&mut buf).await.map_err(|_| AgspError::IoFailure)?;
    toml::from_slice::<ManifestRecord>(&buf).map_err(|_| AgspError::InvalidManifest)
}

pub async fn import(reader: impl AsyncRead + Unpin) -> std::io::Result<()> {
    let decoder = ZstdDecoder::new(BufReader::new(reader));
    let mut tar = Archive::new(decoder.take(MAX_PACKAGE_SIZE as u64));
    tar.unpack("./tmp/extract").await?;
    let manifest = inspect_manifest(File::open("./tmp/extract/MANIFEST").await?);

    Ok(())
}