use async_graphql::{ComplexObject, Context, Object, SimpleObject, Result};
use serde::{Deserialize, Serialize};
use crate::resolvers::resolve_building::resolve_building;

pub struct Query;


#[derive(SimpleObject, Deserialize, Serialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
}

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Building {
    pub id: u32,
    pub name: String,
}

#[ComplexObject]
impl Building {
    async fn rooms(&self) -> Vec<Room> {
        vec![]
    }
}

#[Object]
impl Query {
    async fn building(&self, ctx: &Context<'_>, id: u32) -> Result<Building> {
        return resolve_building(ctx, &id).await;
    }
}