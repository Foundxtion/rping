use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{
    State,
    serde::{Deserialize, Serialize, json::Json},
};

use rocket_krb5::KrbToken;

use crate::types::HostMap;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DnsInfoRequest {
    hostname: String,
}

pub struct ClientGuard {
    ip: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DnsResponse {
    message: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientGuard {
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.client_ip() {
            Some(ip) => Outcome::Success(ClientGuard { ip: ip.to_string() }),
            None => Outcome::Error((Status::BadRequest, "No ip?? wtf".to_string())),
        }
    }
}

#[post("/", format = "application/json", data = "<info>")]
pub async fn post_address(
    info: Json<DnsInfoRequest>,
    client_info: ClientGuard,
    map: &State<HostMap>,
    _token: KrbToken,
) -> Json<DnsResponse> {
    let hostname = info.hostname.clone();
    let ip = client_info.ip;

    map.lock().await.insert(hostname, ip.clone());

    Json::from(DnsResponse {
        message: format!("Saved ip: {}", ip),
    })
}
