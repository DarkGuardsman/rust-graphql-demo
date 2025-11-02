use async_graphql::{ComplexObject, Context, Object, SimpleObject, Result};
use async_graphql::dataloader::DataLoader;
use serde::{Deserialize, Serialize};
use crate::AppContext;
pub(crate) use crate::loaders::room_loader::RoomLoader;
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

    async fn room(&self, ctx: &Context<'_>, id: u32) -> Result<Option<Room>> {
        return resolve_room(ctx, &id).await;
    }
}