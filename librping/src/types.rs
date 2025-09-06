use tabled::Tabled;

#[derive(Tabled)]
pub struct Dns {
    pub hostname: String,
    pub ip: String,
}

impl Dns {
    pub fn new(hostname: String, ip: String) -> Dns {
        Dns { hostname, ip }
    }
}
