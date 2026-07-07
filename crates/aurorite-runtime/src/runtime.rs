use std::sync::Arc;
use tokio::sync::Mutex;
use vismut_core::schemas::ScriptSchema;
use vismut_core::{RegistryError, VismutRuntime, VismutScript};
use crate::RuntimeCtx;

type ArcedCtx = Arc<Mutex<RuntimeCtx>>;

pub struct AuroriteRuntime {
    ctx: ArcedCtx,
    executor: VismutRuntime<ArcedCtx>,
}

impl AuroriteRuntime {
    pub fn new(ctx: Arc<Mutex<RuntimeCtx>>) -> Self {
        let executor = VismutRuntime::new(ctx.clone());
        Self {
            ctx,
            executor,
        }
    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VismutScript<ArcedCtx>, RegistryError> {
        self.executor.parse(schema)
    }
}