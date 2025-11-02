use async_graphql::{Context, Result, Error};
use crate::{Headers};
use crate::schema::Building;

pub async fn resolve_building_list(ctx: &Context<'_>) -> Result<Vec<Building>> {
    let token = ctx.data_opt::<Headers>().map(|headers| &*headers.token).expect("Token not found");
    return get_building_list(token).await;
}

async fn get_building_list(token: &str) -> Result<Vec<Building>> {
    let building_url = std::env::var("BUILDING_URL")?; //TODO move to context.urls.building

    let client = reqwest::Client::new();
    let response = client.get(building_url)
        .header("Authorization", token)
        .send()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    let body: Vec<Building> = response.json()
        .await
        .map_err(|err| Error::new(err.to_string()))?;

    Ok(body)
}