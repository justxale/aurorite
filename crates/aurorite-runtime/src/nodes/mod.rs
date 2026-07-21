use std::sync::Arc;
use parking_lot::Mutex;
use vismut_core::BuiltNode;
use crate::RuntimeCtx;

mod rand;

#[macro_export]
macro_rules! try_extract {
    ($source:expr, $t:path, $name:literal) => {
        if let Some($t(value)) = $source.get($name) {
            value
        } else {
            return Err(vismut_core::ScriptError::MissingInput($name.to_string()));
        }
    };
}

pub(super) type AuroriteCtx = Arc<Mutex<RuntimeCtx>>;
pub(super) type AuroriteNode = BuiltNode<AuroriteCtx>;

pub use rand::build_rand_nodes;