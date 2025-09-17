use std::collections::HashMap;

use reqwest::{Client, Response, StatusCode};

use crate::{
    auth::{
        create_context, derive_principal_from_url, generate_token, prepare_server_token_from_header,
    },
    types::Dns,
};

/// Sends a DNS record to the server using Kerberos authentication.
///
/// ### Parameters
/// - `hostname`: The DNS hostname to send.
/// - `url`: The service URL as a string.
/// - `realm`: The Kerberos realm as a string.
/// - `maybe_retry`: Optional number of retries.
///
/// ### Returns
/// - `Result<(), String>`: Ok if successful, Err with error message otherwise.
///
/// ### Example
/// ```rust
/// send_dns("host1".to_string(), "https://example.com/api".to_string(), "EXAMPLE.COM".to_string(), None).await.unwrap();
/// ```
pub async fn send_dns(
    hostname: String,
    url: String,
    realm: String,
    maybe_retry: Option<usize>,
) -> Result<(), String> {
    let retry = maybe_retry.unwrap_or(5);
    let service_principal = derive_principal_from_url(url.clone(), realm)
        .ok_or("Could not parse principal from url.")?;
    let mut context =
        create_context(service_principal).ok_or("Could not create kerberos client context.")?;

    let client = reqwest::Client::new();
    let mut map = HashMap::new();
    map.insert("hostname", hostname);

    let mut counter = 0;
    let mut status: StatusCode = StatusCode::UNAUTHORIZED;
    let mut server_tok: Option<Vec<u8>> = None;

    loop {
        let client_tok: Option<String> = generate_token(&mut context, server_tok);

        if client_tok.is_none() || status.is_success() {
            break;
        }

        let answer = client
            .post(url.clone())
            .header(
                "Authorization",
                "Negotiate ".to_string() + client_tok.unwrap().as_str(),
            )
            .json(&map)
            .send()
            .await
            .map_err(|e| format!("Send error: {}", e))?;

        counter += 1;
        status = answer.status();

        if status == StatusCode::NOT_FOUND {
            return Err(format!("The url: '{}' is not a valid endpoint", url));
        }

        let header_value = get_header(&answer)?;
        server_tok = prepare_server_token_from_header(header_value);
    }

    if counter >= retry {
        Err(String::from("Retry limit reached, aborting..."))
    } else {
        Ok(())
    }
}

/// Receives a list of DNS records from the server using Kerberos authentication.
///
/// ### Parameters
/// - `url`: The service URL as a string.
/// - `realm`: The Kerberos realm as a string.
/// - `maybe_retry`: Optional number of retries.
///
/// ### Returns
/// - `Result<Vec<Dns>, String>`: Ok with vector of DNS records, Err with error message otherwise.
///
/// ### Example
/// ```rust
/// let dns_list = receive_list("https://example.com/api".to_string(), "EXAMPLE.COM".to_string(), None).await.unwrap();
/// ```
pub async fn receive_list(
    url: String,
    realm: String,
    maybe_retry: Option<usize>,
) -> Result<Vec<Dns>, String> {
    let retry = maybe_retry.unwrap_or(5);
    let service_principal = derive_principal_from_url(url.clone(), realm)
        .ok_or("Could not parse principal from url.")?;
    let mut context =
        create_context(service_principal).ok_or("Could not create kerberos client context.")?;

    let client = reqwest::Client::new();

    let mut counter = 0;
    let mut status: StatusCode = StatusCode::UNAUTHORIZED;
    let mut body: String = String::new();
    let mut server_tok: Option<Vec<u8>> = None;

    loop {
        let client_tok: Option<String> = generate_token(&mut context, server_tok);

        if client_tok.is_none() || status.is_success() {
            break;
        }

        let tok = client_tok.unwrap();

        let answer = send_get(&client, url.clone(), tok).await?;

        counter += 1;
        status = answer.status();

        if status == StatusCode::NOT_FOUND {
            return Err(format!("The url: '{}' is not a valid endpoint", url));
        }

        let header_value = get_header(&answer)?;
        body = get_body(answer).await?;

        server_tok = prepare_server_token_from_header(header_value);
    }

    if counter >= retry {
        return Err(String::from("Retry limit reached, aborting..."));
    }

    let map: HashMap<String, String> = serde_json::from_str::<HashMap<String, String>>(&body)
        .map_err(|e| format!("Parsing error: {}", e))?;

    Ok(map.into_iter().map(|i| Dns::new(i.0, i.1)).collect())
}

/// Extracts the WWW-Authenticate header from a response.
///
/// ### Parameters
/// - `answer`: Reference to a `Response` object.
///
/// ### Returns
/// - `Result<String, String>`: Ok with header value, Err with error message otherwise.
fn get_header(answer: &Response) -> Result<String, String> {
    let headers = {
        let h = answer.headers();
        h.clone()
    };

    let header = headers
        .get("WWW-Authenticate")
        .ok_or("Fetching header error: Could not fetch WWW-Authenticate")?;

    let header_value = header
        .to_str()
        .map_err(|e| format!("Parsing header error: {}", e))?
        .to_string();

    Ok(header_value)
}

/// Asynchronously fetches the body text from a response.
///
/// ### Parameters
/// - `answer`: The `Response` object.
///
/// ### Returns
/// - `Result<String, String>`: Ok with body text, Err with error message otherwise.
async fn get_body(answer: Response) -> Result<String, String> {
    answer
        .text()
        .await
        .map_err(|e| format!("Error fetching body: {}", e))
}

/// Asynchronously sends a GET request with Kerberos token.
///
/// ### Parameters
/// - `client`: Reference to a `Client` object.
/// - `url`: The service URL as a string.
/// - `token`: The Kerberos token as a string.
///
/// ### Returns
/// - `Result<Response, String>`: Ok with response, Err with error message otherwise.
async fn send_get(client: &Client, url: String, token: String) -> Result<Response, String> {
    client
        .get(url.clone())
        .header("Authorization", "Negotiate ".to_string() + &token)
        .send()
        .await
        .map_err(|e| format!("Send error: {}", e))
}
