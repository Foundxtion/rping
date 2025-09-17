use base64::Engine;
use base64::engine::general_purpose;
use libgssapi::context::{SecurityContext, ServerCtx};
use libgssapi::util::Buf;
use rocket::Request;
use rocket::State;
use rocket::futures::lock::Mutex;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::KrbServerCreds;

/// Kerberos token struct, used for authentication in Rocket requests.
pub struct KrbToken {
    pub principal: String,
}

/// Incomplete SPNEGO token struct, used for partial authentication state.
#[derive(Debug)]
pub struct IncompleteSpnego {
    pub token: String,
}

/// Internal struct representing authentication status.
struct AuthStatus {
    krb: Option<KrbToken>,
    spnego: Option<IncompleteSpnego>,
}

impl KrbToken {
    /// Creates a new Kerberos token from a principal string.
    ///
    /// ### Parameters
    /// - `principal`: The Kerberos principal as a string.
    ///
    /// ### Returns
    /// - `KrbToken`: New Kerberos token struct.
    ///
    /// ### Example
    /// ```rust
    /// let token = KrbToken::new("user@EXAMPLE.COM".to_string());
    /// assert_eq!(token.principal, "user@EXAMPLE.COM");
    /// ```
    pub fn new(principal: String) -> KrbToken {
        KrbToken {
            principal: principal,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for KrbToken {
    type Error = String;
    /// Extracts Kerberos token from request headers and validates it.
    ///
    /// ### Parameters
    /// - `request`: Reference to the incoming request.
    ///
    /// ### Returns
    /// - `Outcome<Self, Self::Error>`: Success with token or error outcome.
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = request.headers().get_one("Authorization");

        let server_creds = match request
            .guard::<&State<Mutex<KrbServerCreds>>>()
            .await
            .succeeded()
        {
            Some(t) => t,
            None => {
                return Outcome::Error((
                    Status::InternalServerError,
                    "No Kerberos Server credential state set.".to_string(),
                ));
            }
        };
        let locked_creds = server_creds.lock().await;
        match header {
            None => Outcome::Error((
                Status::Unauthorized,
                "SPNEGO Authentication required.".to_string(),
            )),
            Some(encoded_token) => get_decoded_token(&locked_creds, encoded_token).map_or(
                Outcome::Error((Status::Forbidden, "Principal not allowed".to_string())),
                |auth_status| finalize_response(auth_status, request),
            ),
        }
    }
}

fn finalize_response<'r>(
    auth_status: AuthStatus,
    req: &'r Request<'_>,
) -> Outcome<KrbToken, String> {
    if let Some(spnego) = auth_status.spnego {
        req.local_cache(|| spnego);
    }

    match auth_status.krb {
        Some(token) => Outcome::Success(token),
        None => Outcome::Error((
            Status::Unauthorized,
            "Continuing authentication because of incomplete SPNEGO".to_string(),
        )),
    }
}

fn get_decoded_token(creds: &KrbServerCreds, header_value: &str) -> Option<AuthStatus> {
    let c = creds.creds.clone();
    let mut context = ServerCtx::new(Some(c));

    let token = header_value
        .strip_prefix("Negotiate ")
        .and_then(|b64| general_purpose::STANDARD.decode(b64).ok())?;

    match context.step(&*token) {
        Ok(opt) => wrap_up_token(opt, context),
        Err(e) => {
            println!("There is an error while stepping in server context: {}", e);
            None
        }
    }
}

fn wrap_up_token(maybe_token: Option<Buf>, context: ServerCtx) -> Option<AuthStatus> {
    let principal = get_source_principal(context);
    match maybe_token {
        None => Some(AuthStatus {
            krb: Some(KrbToken::new(principal.unwrap())),
            spnego: None,
        }),
        Some(t) => {
            let encoded_token = general_purpose::STANDARD.encode(&*t).as_str().to_string();

            let krb_token = principal.map_or(None, |p| Some(KrbToken::new(p)));

            Some(AuthStatus {
                krb: krb_token,
                spnego: Some(IncompleteSpnego {
                    token: encoded_token,
                }),
            })
        }
    }
}

fn get_source_principal(mut context: ServerCtx) -> Option<String> {
    if !context.is_complete() {
        return None;
    }
    let principal_cname = context.source_name().ok()?;
    String::from_utf8(principal_cname.display_name().ok()?.to_vec()).ok()
}
