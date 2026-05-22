use aurorite_agsp::export;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs::File;
use tracing_test::traced_test;

static ASSETS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("assets")
});

#[tokio::test]
#[traced_test]
async fn test_success_export() -> Result<(), std::io::Error> {
    let mut res = export(ASSETS_PATH.clone()).await;
    let mut f = File::create("./res.tar.zst").await?;
    tokio::io::copy(&mut res, &mut f).await?;
    Ok(())
}
