use std::collections::HashMap;

use rocket::{State, serde::json::Json};

use crate::types::HostMap;

#[get("/")]
pub async fn get_list(map: &State<HostMap>) -> Json<HashMap<String, String>> {
    let hash_map = map.lock().await.clone();

    Json::from(hash_map)
}
