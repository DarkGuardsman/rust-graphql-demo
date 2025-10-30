use async_graphql::{Context, InputObject, Object};
use crate::resolvers::resolve_hello::resolve_hello;

pub struct Query;

#[derive(InputObject)]
pub struct HelloInput {
    pub message: String,
}

#[Object]
impl Query {
    async fn hello(&self, ctx: &Context<'_>, input: HelloInput) -> String {
        resolve_hello(&ctx, &input)
    }
}