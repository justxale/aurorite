use std::path::{PathBuf};
use std::sync::LazyLock;
use aurorite_arps::{export, AssetRecord, AssetType, ManifestRecord};

static ASSETS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_dir().unwrap().join("tests").join("assets")
});

#[tokio::test]
async fn test() -> Result<(), std::io::Error> {
    let mut should_be = ManifestRecord::new();
    should_be.add_asset_record(
        AssetRecord::new(
            "3dd8d3eb3c14ca12776e102a39b12d1f43a04f0cb3785ca24da22cf1a698a2d1".to_string(),
            "hello.txt".to_string(), AssetType::META
        )
    );

    let res = export(ASSETS_PATH.as_path()).await.unwrap();
    assert!(!res.is_empty());
    Ok(())
}
