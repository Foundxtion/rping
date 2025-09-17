use base64::Engine;
use base64::engine::general_purpose;
use libgssapi::{
    context::{ClientCtx, CtxFlags},
    credential::{Cred, CredUsage},
    name::Name,
    oid::{GSS_MECH_KRB5, GSS_MECH_SPNEGO, GSS_NT_KRB5_PRINCIPAL, OidSet},
};
use reqwest::Url;

pub fn derive_principal_from_url(url: String, realm: String) -> Option<String> {
    let parsed_url = Url::parse(url.as_str()).ok()?;

    let domain = parsed_url
        .domain()
        .map(|s| s.to_string())
        .or_else(|| Some(parsed_url.host().unwrap().to_string()));

    domain.and_then(|s| Some("HTTP/".to_string() + s.as_str() + "@" + realm.as_str()))
}

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

pub fn prepare_server_token_from_header(header: String) -> Option<Vec<u8>> {
    if header.len() < 11 {
        return None;
    }

    let token = header
        .strip_prefix("Negotiate ")
        .and_then(|b64| general_purpose::STANDARD.decode(b64).ok())?;

    Some(token)
}
