use async_graphql::{Context, Result, Error};
use crate::{Headers};
use crate::schema::Building;

pub async fn resolve_building(ctx: &Context<'_>, id: &u32) -> Result<Building> {
    let token = ctx.data_opt::<Headers>().map(|headers| &*headers.token);
    if token.is_none() {
        Err(Error::new("Unauthorized"))?
    }
    let token = token.unwrap();
    return get_building(id, token).await;
}

async fn get_building(id: &u32, token: &str) -> Result<Building> {
    let building_url = std::env::var("BUILDING_URL")?; //TODO move to context.urls.building

    let client = reqwest::Client::new();
    let response = client.get(format!("{}/{}", building_url, id))
        .header("Authorization", token)
        .send()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    let body: Building = response.json()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    Ok(body)
}