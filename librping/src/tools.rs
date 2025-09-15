use crate::{
    client::{receive_list, send_dns},
    display::display_dns,
};

pub async fn list(url: String) {
    let dns = receive_list(url).await;

    display_dns(dns)
}

pub async fn send(url: String) {
    let hostname = hostname::get().unwrap();
    send_dns(hostname.into_string().unwrap(), url).await;
}
