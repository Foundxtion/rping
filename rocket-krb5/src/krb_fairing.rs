use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Status,
};

use crate::IncompleteSpnego;

pub struct KrbFairing {}

#[rocket::async_trait]
impl Fairing for KrbFairing {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Authentication guard for Kerberos SPNEGO",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let spnego = request.local_cache(|| IncompleteSpnego {
            token: String::new(),
        });

        if response.status() == Status::Unauthorized {
            response.set_raw_header(
                "WWW-Authenticate",
                "Negotiate ".to_string() + spnego.token.as_str(),
            );
            return;
        }

        if !spnego.token.is_empty() {
            response.set_raw_header(
                "WWW-Authenticate",
                "Negotiate ".to_string() + spnego.token.as_str(),
            );
        }
    }
}
