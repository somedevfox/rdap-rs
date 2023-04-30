use crate::{
    model::{self, ObjectClassType},
    result::{Result, Error},
    Domain,
};

pub struct Endpoints {
    pub domain: String,
    pub autnum: String,
}
pub struct Registry {
    domain: String,
    endpoints: Endpoints,
}
impl Registry {
    pub fn connect(domain: impl Into<String>, endpoints: Endpoints) -> Self {
        Self {
            domain: domain.into(),
            endpoints,
        }
    }

    /// Connect to Réseaux IP Européens Network Coordination Centre.
    pub fn ripe() -> Self {
        Registry::connect(
            "rdap.db.ripe.net",
            Endpoints {
                domain: String::from("/domain"),
                autnum: String::from("/autnum"),
            },
        )
    }

    pub fn query_domain(&self, domain_name: impl ToString) -> Result<Domain> {
        let registry_record = ureq::get(
            format!(
                "https://{}{}/{}",
                self.domain,
                self.endpoints.domain,
                domain_name.to_string()
            )
            .as_str(),
        )
        .call()?
        .into_json::<model::RegistryRecord>()?;
        if registry_record.record_type == ObjectClassType::Domain {
            Ok(Domain::from(registry_record))
        } else {
            Err(Error::NotDomain)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{sync::{Endpoints, Registry}, IpVersion};
    #[test]
    fn ripe() {
        let registry = Registry::connect(
            "rdap.db.ripe.net",
            Endpoints {
                domain: String::from("/domain"),
                autnum: String::from("/autnum"),
            },
        );
        let domain = registry.query_domain("193.0.6.139.in-addr.arpa").unwrap();
        assert_eq!(domain.rir_domain, String::from("whois.ripe.net"));
        assert_eq!(domain.name, String::from("6.139.in-addr.arpa"));
        assert_eq!(domain.country.code, String::from("DE"));
        assert_eq!(domain.country.name, String::from("Germany"));
        assert_eq!(domain.network.name, String::from("IWZ-LAN"));
        assert_eq!(domain.network.ip_version, IpVersion::V4);
    }
}
