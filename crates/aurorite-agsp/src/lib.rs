pub(crate) mod checksum;
mod common;
mod exporter;
mod importer;
pub mod fs;
#[cfg(test)]
mod tests;
pub mod consts;

pub use common::{AssetRecord, AssetType, MAX_PACKAGE_SIZE, ManifestRecord};
pub use exporter::export;
pub use importer::{AgspError, import};

pub static VERSION: &str = env!("CARGO_PKG_VERSION");