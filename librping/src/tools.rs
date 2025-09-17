use crate::{
    client::{receive_list, send_dns},
    display::display_dns,
};

pub async fn list(url: String, realm: String) {
    match receive_list(url, realm, None).await {
        Ok(dns) => display_dns(dns),
        Err(e) => println!("{}", e),
    }
}

pub async fn send(url: String, realm: String) {
    let hostname = hostname::get().unwrap();
    send_dns(hostname.into_string().unwrap(), url, realm, None)
        .await
        .err()
        .inspect(|e| println!("{}", e));
}
