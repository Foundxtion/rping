use crate::{
    client::{receive_list, send_dns},
    display::display_dns,
};

/// Lists DNS records from the server and displays them.
///
/// # Parameters
/// - `url`: The service URL as a string.
/// - `realm`: The Kerberos realm as a string.
///
/// # Example
/// ```rust
/// list("https://example.com/api".to_string(), "EXAMPLE.COM".to_string()).await;
/// ```
pub async fn list(url: String, realm: String) {
    match receive_list(url, realm, None).await {
        Ok(dns) => display_dns(dns),
        Err(e) => println!("{}", e),
    }
}

/// Sends the current hostname as a DNS record to the server.
///
/// # Parameters
/// - `url`: The service URL as a string.
/// - `realm`: The Kerberos realm as a string.
///
/// # Example
/// ```rust
/// send("https://example.com/api".to_string(), "EXAMPLE.COM".to_string()).await;
/// ```
pub async fn send(url: String, realm: String) {
    let hostname = hostname::get().unwrap();
    send_dns(hostname.into_string().unwrap(), url, realm, None)
        .await
        .err()
        .inspect(|e| println!("{}", e));
}
