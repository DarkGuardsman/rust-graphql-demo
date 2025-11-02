use async_graphql::extensions::{
    Extension,
    ExtensionContext,
    ExtensionFactory,
    NextExecute,
};
use async_graphql::{Value};
use std::sync::Arc;
use byte_unit::{Byte};
use log::{debug, trace};
use memory_stats::memory_stats;

pub struct MemoryMetricsExtension;

impl ExtensionFactory for MemoryMetricsExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(MemoryMetricsExtension)
    }
}

#[async_trait::async_trait]
impl Extension for MemoryMetricsExtension {

    // Crude way to get memory usage for tracking
    async fn execute(&self, ctx: &ExtensionContext<'_>, operation_name: Option<&str>, next: NextExecute<'_>) -> async_graphql::Response {

        let memory_before_stat = memory_stats().unwrap();
        let memory_before = memory_before_stat.physical_mem + memory_before_stat.virtual_mem;
        trace!("Memory usage before execution: {} bytes", memory_before);

        let result = next.run(ctx, operation_name).await;

        let memory_after_stat = memory_stats().unwrap();
        let memory_after = memory_after_stat.physical_mem + memory_after_stat.virtual_mem;
        trace!("Memory usage after execution: {} bytes", memory_after);

        let memory_diff = memory_after as i64 - memory_before as i64;
        debug!("Memory difference during execution: {} bytes", memory_diff);

        tracing::Span::current().record("memory_allocated_bytes", memory_diff); // TODO this isn't consistent due to memory released before call

        let byte_instance = Byte::from_i64(memory_diff).unwrap();
        result.extension("memory", Value::String(format!("{byte_instance:#}")))
    }
}