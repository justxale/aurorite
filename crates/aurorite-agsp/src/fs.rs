use aurorite_util::env;
use std::io::Error;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader};

#[inline]
pub async fn read_text_file(path: impl AsRef<Path>) -> Result<String, Error> {
    match File::open(&env().data_root.join(path)).await {
        Ok(mut f) => {
            let mut content = String::new();
            match f.read_to_string(&mut content).await {
                Ok(_) => Ok(content),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

#[inline]
pub async fn write_file(path: impl AsRef<Path>, bytes: impl AsyncRead) -> Result<(), Error> {
    match File::create_new(&env().data_root.join(path)).await {
        Ok(mut f) => {
            let buf = BufReader::new(bytes);
            f.write_all(buf.buffer()).await.map(|_| ())
        },
        Err(e) => Err(e)
    }
}

#[inline]
pub async fn delete_file(path: impl AsRef<Path>) -> Result<(), Error> {
    tokio::fs::remove_file(path).await
}