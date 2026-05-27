use crate::{export, import};
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs::File;
use tracing_test::traced_test;

static ASSETS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .join("src")
        .join("tests")
        .join("assets")
});

#[tokio::test]
#[traced_test]
async fn test_agsp_io() -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all("./tmpdir").await?;

    let mut res = export(ASSETS_PATH.clone()).await;
    let mut f = File::create("./tmpdir/res.tar.zst").await?;
    tokio::io::copy(&mut res, &mut f).await?;

    let f = File::open("./tmpdir/res.tar.zst").await?;
    let results = import(f).await.unwrap();
    for res in results {
        res.unwrap();
    }
    tokio::fs::remove_dir_all("./tmpdir").await?;
    tokio::fs::remove_dir_all("./.tmp").await?;
    tokio::fs::remove_dir_all("./packages").await?;
    Ok(())
}
