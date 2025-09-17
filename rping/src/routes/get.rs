use std::collections::HashMap;

use rocket::{State, serde::json::Json};
use rocket_krb5::KrbToken;

use crate::types::HostMap;

#[doc = r"Handles GET requests to retrieve all DNS records."]
#[doc = r""]
#[doc = r"### Parameters"]
#[doc = r#"- `map`: Shared state containing DNS records."#]
#[doc = r#"- `token`: Kerberos token for authentication."#]
#[doc = r""]
#[doc = r"### Returns"]
#[doc = r#"- `Json<HashMap<String, String>>`: Map of hostnames to IPs."#]
#[doc = r""]
#[doc = r"### Example"]
#[doc = r#""#]
#[doc = r#""#]
#[doc = r#"```rust"#]
#[doc = r#""#]
#[doc = r#""#]
#[doc = r#"// Usage in Rocket route"#]
#[doc = r#"#[get("/")] "#]
#[doc = r#"async fn get_list(map: &State<HostMap>, token: KrbToken) -> Json<HashMap<String, String>> {"#]
#[doc = r#"    // ... "#]
#[doc = r#" } "#]
#[doc = r#""#]
#[doc = r#""#]
#[get("/")]
pub async fn get_list(map: &State<HostMap>, token: KrbToken) -> Json<HashMap<String, String>> {
    let hash_map = map.lock().await.clone();

    println!("{}", token.principal);

    Json::from(hash_map)
}
