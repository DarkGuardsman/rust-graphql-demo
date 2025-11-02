use std::collections::HashMap;
use async_graphql::dataloader::Loader;
use async_graphql::Error;
use crate::datasource::rooms::get_room_list;
use crate::schema::{Room};

pub struct RoomLoader {
    pub(crate) token: String
}

#[cfg_attr(feature = "boxed-trait", async_trait::async_trait)]
impl Loader<u32> for RoomLoader {
    type Value = Room;
    type Error = Error;
    
    async fn load(&self, keys: &[u32]) -> Result<HashMap<u32, Self::Value>, Self::Error> {
        let rooms = get_room_list(keys, &*self.token).await?;
        return Ok(rooms
            .into_iter()
            .map(|n| (n.id, n))
            .collect());
    }
}