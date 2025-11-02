use async_graphql::{Context, Result, Error};
use crate::{AppContext, Headers};
use crate::schema::Room;

pub async fn resolve_room(ctx: &Context<'_>, id: &u32) -> Result<Option<Room>> {
    
    // Use DataLoader if available
    let app_context = ctx.data::<AppContext>()?;
    if app_context.use_room_loader {
        return Ok(app_context.room_loader.load_one(*id).await?);
    }
    
    let token = ctx.data_opt::<Headers>().map(|headers| &*headers.token).expect("Token not found");
    return get_room(id, token).await;
}

async fn get_room(id: &u32, token: &str) -> Result<Option<Room>> {
    let building_url = std::env::var("ROOM_URL")?; //TODO move to context.urls.room

    let client = reqwest::Client::new();
    let response = client.get(format!("{}/{}", building_url, id))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    let body: Room = response.json()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    Ok(Some(body))
}