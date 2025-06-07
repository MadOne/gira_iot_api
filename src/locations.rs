use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub id: Option<u16>,
    pub parent_location: Option<u16>,
    pub displayName: String,
    pub functions: Option<Vec<String>>,
    pub locationType: String,
    pub locations: Option<Vec<u16>>,
}

#[derive(Clone, Debug)]
pub struct Locations {
    pub locations: Arc<Mutex<HashMap<u16, Location>>>,
    pub set: Arc<Mutex<bool>>,
}

impl Locations {
    pub async fn get(&self, id: u16) -> Location {
        self.locations
            .lock()
            .await
            .get(&id)
            .expect("error getting location")
            .clone()
    }
}
