use async_graphql::Context;
use crate::Headers;
use crate::schema::HelloInput;

pub fn resolve_hello(ctx: &Context<'_>, input: &HelloInput) -> String {
    let user = ctx.data_opt::<Headers>().map(|headers| &*headers.user).unwrap_or_else(|| "User");
    format!("Hello, {}! {}", user, input.message).to_string()
}