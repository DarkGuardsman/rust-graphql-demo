use async_graphql::{Context, Result};
use crate::{AppContext, DataLoaders, Headers};
use crate::schema::Room;
use crate::datasource::rooms::get_room;

pub async fn resolve_room(ctx: &Context<'_>, id: &u32) -> Result<Option<Room>> {

    // Use DataLoader if enabled
    let app_context = ctx.data::<AppContext>()?;
    let loaders = ctx.data::<DataLoaders>()?;
    if app_context.use_room_loader {
        return Ok(loaders.room_loader.load_one(*id).await?);
    }

    let token = ctx.data_opt::<Headers>().map(|headers| &*headers.token).expect("Token not found");
    return get_room(id, token).await;
}