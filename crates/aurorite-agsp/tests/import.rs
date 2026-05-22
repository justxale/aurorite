use aurorite_agsp::{AgspError, import};
use tokio::fs::File;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn test_success_import() -> Result<(), AgspError> {
    let f = File::open("./res.zstd").await?;
    let results = import(f).await?;
    for res in results {
        res.unwrap();
    }
    Ok(())
}
