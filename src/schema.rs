use async_graphql::{ComplexObject, Context, Object, SimpleObject, Result};
use serde::{Deserialize, Serialize};
use crate::resolvers::resolve_building::resolve_building;
use crate::resolvers::resolve_building_list::resolve_building_list;
use crate::resolvers::resolve_room::resolve_room;

pub struct Query;


#[derive(SimpleObject, Deserialize, Serialize, Clone)]
pub struct Room {
    pub id: u32,
    pub name: String,
}

#[derive(SimpleObject, Deserialize, Serialize, Clone)]
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

    async fn buildings(&self, ctx: &Context<'_>) -> Result<Vec<Building>> {
        return resolve_building_list(ctx).await;
    }

    async fn room(&self, ctx: &Context<'_>, id: u32) -> Result<Room> {
        return resolve_room(ctx, &id).await;
    }
}