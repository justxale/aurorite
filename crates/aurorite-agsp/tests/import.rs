use aurorite_agsp::{export, import};
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs::File;
use tokio::io::DuplexStream;
use tracing_test::traced_test;

static ASSETS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("assets")
});

#[tokio::test]
#[traced_test]
async fn test_success_import() -> Result<(), std::io::Error> {
    let f = File::open("./res.zstd").await?;
    import(f).await?;
    assert!(false);
    Ok(())
}
