use aurorite_util::env;
use std::io::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[inline]
pub async fn load_text_file(path: impl AsRef<Path>) -> Result<String, Error> {
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
