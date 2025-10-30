use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "Hello, async-graphql with Axum!".to_string()
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;