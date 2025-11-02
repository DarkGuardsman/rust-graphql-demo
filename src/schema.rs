use async_graphql::{ComplexObject, Context, Object, SimpleObject, Result};
use serde::{Deserialize, Serialize};
pub(crate) use crate::loaders::room_loader::RoomLoader;
use crate::resolvers::resolve_building::resolve_building;
use crate::resolvers::resolve_building_list::resolve_building_list;
use crate::resolvers::resolve_room::resolve_room;

pub struct Query;

#[derive(SimpleObject, Deserialize, Serialize, Clone)]
#[graphql(complex)]
pub struct Room {
    pub id: u32,
    pub name: String,

    #[graphql(visible = false)]
    #[serde(rename = "buildingId")]
    pub building_id: u32,
}

#[ComplexObject]
impl Room {
    async fn building(&self, ctx: &Context<'_>) -> Result<Option<Building>> {
        return resolve_building(ctx, &self.building_id).await;
    }
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
        vec![] //TODO implement as a look-ahead modifier on get_building by sending `?expand=rooms` with only `ids` returning
    }
}


#[Object]
impl Query {

    async fn building(&self, ctx: &Context<'_>, id: u32) -> Result<Option<Building>> {
        return resolve_building(ctx, &id).await;
    }

    async fn buildings(&self, ctx: &Context<'_>) -> Result<Vec<Building>> {
        return resolve_building_list(ctx).await;
    }

    async fn room(&self, ctx: &Context<'_>, id: u32) -> Result<Option<Room>> {
        return resolve_room(ctx, &id).await;
    }

    #[graphql(entity)]
    async fn resolve_room_entity_by_id(&self, ctx: &Context<'_>, id: u32) -> Result<Option<Room>> {
        return resolve_room(ctx, &id).await;
    }

    #[graphql(entity)]
    async fn resolve_building_entity_by_id(&self, ctx: &Context<'_>, id: u32) -> Result<Option<Building>> {
        return resolve_building(ctx, &id).await;
    }
}