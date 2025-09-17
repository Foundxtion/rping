use libgssapi::{
    credential::{Cred, CredUsage},
    name::Name,
    oid::{GSS_MECH_KRB5, GSS_MECH_SPNEGO, GSS_NT_KRB5_PRINCIPAL, OidSet},
};

/// Kerberos server credentials struct, used for accepting and validating Kerberos tokens.
/// Stores principal, credentials, and canonicalized name for the server.
pub struct KrbServerCreds {
    pub principal: String,
    pub creds: Cred,
    pub name: Name,
}

impl KrbServerCreds {
    /// Creates new Kerberos server credentials from a principal string.
    ///
    /// ### Parameters
    /// - `principal`: The Kerberos principal as a string.
    ///
    /// ### Returns
    /// - `Option<KrbServerCreds>`: New server credentials struct, or None if creation fails.
    ///
    /// ### Example
    /// ```rust
    /// let creds = KrbServerCreds::new("HTTP/server@EXAMPLE.COM".to_string());
    /// assert!(creds.is_some());
    /// ```
    pub fn new(principal: String) -> Option<KrbServerCreds> {
        let name = Name::new(principal.as_bytes(), Some(&GSS_NT_KRB5_PRINCIPAL)).ok()?;
        let cname = name.canonicalize(Some(&GSS_MECH_KRB5)).ok()?;
        let mut desired = OidSet::new().ok()?;
        desired.add(&GSS_MECH_SPNEGO).ok()?;

        let creds = match Cred::acquire(Some(&cname), None, CredUsage::Accept, Some(&desired)) {
            Ok(s) => Some(s),
            Err(e) => {
                println!("{}", e);
                None
            }
        }?;

        Some(KrbServerCreds {
            principal,
            creds,
            name: cname,
        })
    }
}
