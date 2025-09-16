use libgssapi::{
    credential::{Cred, CredUsage},
    name::Name,
    oid::{GSS_MECH_KRB5, GSS_MECH_SPNEGO, GSS_NT_KRB5_PRINCIPAL, OidSet},
};

pub struct KrbServerCreds {
    pub principal: String,
    pub creds: Cred,
    pub name: Name,
}

impl KrbServerCreds {
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
