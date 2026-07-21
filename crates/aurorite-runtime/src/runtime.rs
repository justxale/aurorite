use std::sync::Arc;
use parking_lot::Mutex;
use vismut_core::schemas::ScriptSchema;
use vismut_core::{RegistryError, VismutRuntime, VismutScript};
use vismut_core::nodes::build_math_nodes;
use crate::nodes::build_rand_nodes;
use crate::RuntimeCtx;

type ArcedCtx = Arc<Mutex<RuntimeCtx>>;
static INCLUSION_ERROR: &str = "failed to start Vismut runtime";

pub struct AuroriteRuntime {
    ctx: ArcedCtx,
    executor: VismutRuntime<ArcedCtx>,
}

impl AuroriteRuntime {
    pub fn new(ctx: Arc<Mutex<RuntimeCtx>>) -> Self {
        let mut executor = VismutRuntime::new(ctx.clone());
        executor.include(build_math_nodes()).expect(INCLUSION_ERROR);
        executor.include(build_rand_nodes()).expect(INCLUSION_ERROR);
        Self {
            ctx,
            executor,
        }
    }

    pub fn parse(&self, schema: &ScriptSchema) -> Result<VismutScript<ArcedCtx>, RegistryError> {
        self.executor.parse(schema)
    }
}