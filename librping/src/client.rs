use std::collections::HashMap;

use crate::types::Dns;

pub async fn send_dns(hostname: String, url: String) {
    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("hostname", hostname);

    let _ = client.post(url).json(&map).send().await;
}

pub async fn receive_list(url: String) -> Vec<Dns> {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    let map: HashMap<String, String> =
        serde_json::from_str::<HashMap<String, String>>(body.as_str())
            .ok()
            .unwrap();

    map.into_iter().map(|i| Dns::new(i.0, i.1)).collect()
}
