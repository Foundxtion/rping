use tabled::Tabled;

/// Represents a DNS entry with a hostname and IP address.
/// Used for storing and displaying DNS records in the application.
#[derive(Tabled)]
pub struct Dns {
    pub hostname: String,
    pub ip: String,
}

impl Dns {
    /// Creates a new `Dns` struct from hostname and IP address strings.
    ///
    /// ### Parameters
    /// - `hostname`: The DNS hostname as a string.
    /// - `ip`: The IP address as a string.
    ///
    /// ### Returns
    /// - `Dns`: A new DNS record struct.
    ///
    /// ### Example
    /// ```rust
    /// let dns = Dns::new("host1".to_string(), "192.168.1.1".to_string());
    /// assert_eq!(dns.hostname, "host1");
    /// assert_eq!(dns.ip, "192.168.1.1");
    /// ```
    pub fn new(hostname: String, ip: String) -> Dns {
        Dns { hostname, ip }
    }
}
