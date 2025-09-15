use rocket::{
    Request, Response,
    fairing::{Fairing, Info, Kind},
    http::Status,
};

pub struct KrbFairing {}

#[rocket::async_trait]
impl Fairing for KrbFairing {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Authentication guard for Kerberos SPNEGO",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status() != Status::Unauthorized {
            return;
        }

        response.set_raw_header("WWW-Authenticate", "Negotiate");
    }
}
