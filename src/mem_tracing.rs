use async_graphql::extensions::{
    Extension,
    ExtensionContext,
    ExtensionFactory,
    NextExecute,
    NextResolve,
    ResolveInfo,
    NextValidation
};
use async_graphql::{ServerError, Result};
use std::sync::Arc;
use tracing::info;
use memory_stats::memory_stats;

pub struct MemoryMetricsExtension;

impl ExtensionFactory for MemoryMetricsExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(MemoryMetricsExtension)
    }
}

#[async_trait::async_trait]
impl Extension for MemoryMetricsExtension {
    // You can use different hooks like `execution_started`, `resolve_field`, etc.
    // For request-level memory usage, the `execute` hook is a good place.
    async fn execute(&self, ctx: &ExtensionContext<'_>, operation_name: Option<&str>, next: NextExecute<'_>) -> async_graphql::Response {
        // Record memory usage BEFORE execution
        let memory_before = memory_stats().unwrap().physical_mem;
        info!("Memory usage before execution: {} bytes", memory_before);

        let result = next.run(ctx, operation_name).await;

        let memory_after = memory_stats().unwrap().physical_mem;
        info!("Memory usage after execution: {} bytes", memory_after);

        let memory_diff = memory_after as i64 - memory_before as i64;
        info!("Memory difference during execution: {} bytes", memory_diff);

        tracing::Span::current().record("memory_allocated_bytes", memory_diff);

        result
    }
}