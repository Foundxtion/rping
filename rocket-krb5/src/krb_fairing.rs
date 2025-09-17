use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Status,
};

use crate::IncompleteSpnego;

/// Fairing for Kerberos SPNEGO authentication in Rocket.
/// Used to attach authentication headers to responses.
pub struct KrbFairing {}

#[rocket::async_trait]
impl Fairing for KrbFairing {
    /// Returns information about the fairing, including its name and kind.
    ///
    /// ### Returns
    /// - `Info`: Fairing information struct.
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Authentication guard for Kerberos SPNEGO",
            kind: Kind::Response,
        }
    }

    /// Handles the response, attaching Kerberos authentication headers if needed.
    ///
    /// ### Parameters
    /// - `request`: Reference to the incoming request.
    /// - `response`: Mutable reference to the outgoing response.
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
