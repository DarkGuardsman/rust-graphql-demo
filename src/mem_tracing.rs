use async_graphql::extensions::{
    Extension,
    ExtensionContext,
    ExtensionFactory,
    NextExecute,
};
use async_graphql::{Value};
use std::sync::Arc;
use alloc_metrics::{global_metrics};
use byte_unit::{Byte};
use log::{debug, trace};

pub struct MemoryMetricsExtension;

impl ExtensionFactory for MemoryMetricsExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(MemoryMetricsExtension)
    }
}

#[derive(serde::Serialize)]
struct ExtensionMemoryStats {
    memory: String,
    bytes: isize,
    count: isize,
}


#[async_trait::async_trait]
impl Extension for MemoryMetricsExtension {

    // Crude way to get memory usage for tracking
    async fn execute(&self, ctx: &ExtensionContext<'_>, operation_name: Option<&str>, next: NextExecute<'_>) -> async_graphql::Response {

        let before = global_metrics();
        trace!("Allocations before: {} bytes {} count", before.allocated_bytes, before.allocations);

        let result = next.run(ctx, operation_name).await;

        let after = global_metrics();
        trace!("Allocations after: {} bytes {} count", after.allocated_bytes, after.allocations);

        let delta = after - before;
        debug!("Allocations: {} bytes {} count", delta.allocated_bytes, delta.allocations);

        tracing::Span::current().record("memory_allocated_bytes", delta.allocated_bytes);
        tracing::Span::current().record("memory_allocated_count", delta.allocations);

        let byte_instance = Byte::from_u64(delta.allocated_bytes as u64);
        let stats = ExtensionMemoryStats {
            memory: format!("{byte_instance:#}"),
            bytes: delta.allocated_bytes,
            count: delta.allocations,
        };
        let json_value = serde_json::to_value(&stats).unwrap();
        let gql_value = Value::from_json(json_value).unwrap();
        result.extension("memory", gql_value)
    }
}