use async_graphql::{Error, Result};
use reqwest::Client;
use crate::schema::Room;

pub async fn get_room(id: &u32, token: &str) -> Result<Option<Room>> {
    let room_url = std::env::var("ROOM_URL")?; //TODO move to context.urls.room

    let client = Client::new();
    let response = client.get(format!("{}/{}", room_url, id))
        .header("Authorization", token)
        .send()
        .await?;

    let body: Room = response.json()
        .await?;

    Ok(Some(body))
}

pub async fn get_room_list(ids: &[u32], token: &str) -> Result<Vec<Room>> {
    let room_url = std::env::var("ROOM_URL")?; //TODO move to context.urls.room

    let client = Client::new();
    let params: Vec<(&str, String)> = ids.iter()
        .map(|id| ("id", id.to_string()))
        .collect();

    let response_result = client.get(room_url)
        .query(&params)
        .header("Authorization", token)
        .send()
        .await;

    if response_result.is_err() {
        log::error!("Buildings | api:rooms | error: {}", response_result.err().unwrap().to_string());
        return Err(Error::new("Room request failed"));
    }

    let response = response_result?;
    let status = response.status();
    if status != 200 {
        log::error!("Buildings | api:rooms | status: {}", status);
        return Err(Error::new(status.to_string()));
    }

    let body: Vec<Room> = response.json()
        .await?;

    Ok(body)
}