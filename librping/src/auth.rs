use base64::Engine;
use base64::engine::general_purpose;
use libgssapi::{
    context::{ClientCtx, CtxFlags},
    credential::{Cred, CredUsage},
    name::Name,
    oid::{GSS_MECH_KRB5, GSS_MECH_SPNEGO, GSS_NT_KRB5_PRINCIPAL, OidSet},
};
use reqwest::Url;

/// Derives a Kerberos principal string from a given URL and realm.
///
/// ### Parameters
/// - `url`: The service URL as a string.
/// - `realm`: The Kerberos realm as a string.
///
/// ### Returns
/// - `Option<String>`: The derived principal string in the format `HTTP/<domain>@<realm>`, or `None` if parsing fails.
///
/// ### Example
/// ```rust
/// let principal = derive_principal_from_url("https://example.com".to_string(), "EXAMPLE.COM".to_string());
/// assert_eq!(principal, Some("HTTP/example.com@EXAMPLE.COM".to_string()));
/// ```
pub fn derive_principal_from_url(url: String, realm: String) -> Option<String> {
    let parsed_url = Url::parse(url.as_str()).ok()?;

    let domain = parsed_url
        .domain()
        .map(|s| s.to_string())
        .or_else(|| Some(parsed_url.host().unwrap().to_string()));

    domain.and_then(|s| Some("HTTP/".to_string() + s.as_str() + "@" + realm.as_str()))
}

/// Creates a GSSAPI client context for the given service principal name.
///
/// ### Parameters
/// - `service_name`: The Kerberos principal name of the service as a string.
///
/// ### Returns
/// - `Option<ClientCtx>`: A new GSSAPI client context, or `None` if creation fails.
///
/// ### Example
/// ```rust
/// let ctx = create_context("HTTP/example.com@EXAMPLE.COM".to_string());
/// assert!(ctx.is_some());
/// ```
pub fn create_context(service_name: String) -> Option<ClientCtx> {
    let mechs = {
        let mut s = OidSet::new().ok()?;
        s.add(&GSS_MECH_SPNEGO).ok()?;
        s
    };

    let creds = Cred::acquire(None, None, CredUsage::Initiate, Some(&mechs)).ok()?;

    let name = Name::new(service_name.as_bytes(), Some(&GSS_NT_KRB5_PRINCIPAL)).ok()?;
    let cname = name.canonicalize(Some(&GSS_MECH_KRB5)).ok()?;

    Some(ClientCtx::new(
        Some(creds),
        cname,
        CtxFlags::GSS_C_MUTUAL_FLAG,
        Some(&GSS_MECH_SPNEGO),
    ))
}

/// Generates a GSSAPI token for authentication, encoding it in base64.
///
/// ### Parameters
/// - `context`: Mutable reference to a GSSAPI client context.
/// - `server_token`: Optional server token as a byte vector.
///
/// ### Returns
/// - `Option<String>`: The base64-encoded token, or `None` if generation fails.
///
/// ### Example
/// ```rust
/// let mut ctx = create_context("HTTP/example.com@EXAMPLE.COM".to_string()).unwrap();
/// let token = generate_token(&mut ctx, None);
/// assert!(token.is_some());
/// ```
pub fn generate_token(context: &mut ClientCtx, server_token: Option<Vec<u8>>) -> Option<String> {
    // we do an unwrap because we want a panic in case of a context error.
    let token = match context.step(server_token.as_deref(), None) {
        Ok(opt) => opt,
        Err(e) => {
            println!("Step error: {}", e);
            None
        }
    }?;
    let encoded_token = general_purpose::STANDARD.encode(&*token);

    Some(encoded_token)
}

/// Prepares a server token from an HTTP Negotiate header by decoding its base64 content.
///
/// ### Parameters
/// - `header`: The HTTP Negotiate header as a string.
///
/// ### Returns
/// - `Option<Vec<u8>>`: The decoded server token as a byte vector, or `None` if decoding fails.
///
/// ### Example
/// ```rust
/// let header = "Negotiate dG9rZW4=".to_string();
/// let token = prepare_server_token_from_header(header);
/// assert!(token.is_some());
/// ```
pub fn prepare_server_token_from_header(header: String) -> Option<Vec<u8>> {
    if header.len() < 11 {
        return None;
    }

    let token = header
        .strip_prefix("Negotiate ")
        .and_then(|b64| general_purpose::STANDARD.decode(b64).ok())?;

    Some(token)
}
