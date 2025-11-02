use std::collections::HashMap;
use std::convert::Infallible;
use async_graphql::dataloader::Loader;
use crate::schema::{Room};

pub struct RoomLoader;

#[cfg_attr(feature = "boxed-trait", async_trait::async_trait)]
impl Loader<u32> for RoomLoader {
    type Value = Room;
    type Error = Infallible;

    async fn load(&self, keys: &[u32]) -> Result<HashMap<u32, Self::Value>, Self::Error> {
        Ok(keys.iter().copied().map(|n| (n, Room {id: n, name: n.to_string()})).collect())
    }
}