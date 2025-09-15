use base64::Engine;
use base64::engine::general_purpose;
use libgssapi::context::{SecurityContext, ServerCtx};
use rocket::Request;
use rocket::State;
use rocket::futures::lock::Mutex;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::KrbServerCreds;

pub struct KrbToken {
    pub principal: String,
}

impl KrbToken {
    pub fn new(principal: String) -> KrbToken {
        KrbToken {
            principal: principal,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for KrbToken {
    type Error = String;
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
                Outcome::Error((Status::Unauthorized, "Principal not allowed".to_string())),
                |tok| Outcome::Success(tok),
            ),
        }
    }
}

fn get_decoded_token(creds: &KrbServerCreds, header_value: &str) -> Option<KrbToken> {
    let c = creds.creds.clone();
    println!("{:?}", c);
    let mut context = ServerCtx::new(Some(c));
    println!("{:?}", context);
    let mut validated_context = header_value
        .strip_prefix("Negotiate ")
        .and_then(|b64| general_purpose::STANDARD.decode(b64.as_bytes()).ok())
        .and_then(move |token| {
            // if we have another token to process, then it should fail, SPNEGO should not require
            // to send another token to the client
            match context.step(&*token) {
                Ok(opt) => match opt {
                    Some(_) => {
                        println!("There is another token to send wtf");
                        None
                    }
                    None => Some(context),
                },
                Err(e) => {
                    println!("There is an error while stepping in server context: {}", e);
                    None
                }
            }
        })?;

    let principal_cname = validated_context.target_name().ok()?;
    let principal = String::from_utf8(principal_cname.display_name().ok()?.to_vec()).ok()?;

    Some(KrbToken::new(principal))
}
