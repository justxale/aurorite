use sha2::Digest;
use tokio_stream::{Stream, StreamExt};
use tokio_util::bytes::Bytes;
use crate::AgspError;

pub async fn compute_hash<S: Stream<Item = Result<Bytes, std::io::Error>> + Unpin>(mut stream: S) -> Result<String, AgspError> {
    let mut hash = sha2::Sha256::new();
    while let Some(bytes) = stream.next().await {
        hash.update(&bytes?);   
    }
    Ok(const_hex::encode(hash.finalize()))
}