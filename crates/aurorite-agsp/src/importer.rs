use tokio::io::{BufReader, DuplexStream};
use async_compression::tokio::bufread::ZstdDecoder;
use tokio_tar::Archive;

pub async fn import(stream: DuplexStream) {
    let decoder = ZstdDecoder::new(BufReader::new(stream));
    let tar = Archive::new(decoder);
    todo!("implement importing")
}