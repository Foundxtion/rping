use std::collections::HashMap;

use rocket::{State, serde::json::Json};
use rocket_krb5::KrbToken;

use crate::types::HostMap;

#[get("/")]
pub async fn get_list(map: &State<HostMap>, token: KrbToken) -> Json<HashMap<String, String>> {
    let hash_map = map.lock().await.clone();

    println!("{}", token.principal);

    Json::from(hash_map)
}
