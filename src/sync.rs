use crate::{
    model::{self, ObjectClassType},
    Country, Domain,
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

    pub fn query_domain(&self, domain_name: impl ToString) {
        let registry_record = ureq::get(
            format!(
                "https://{}{}/{}",
                self.domain,
                self.endpoints.domain,
                domain_name.to_string()
            )
            .as_str(),
        )
        .call()
        .unwrap()
        .into_json::<model::RegistryRecord>()
        .unwrap();
        if registry_record.record_type == ObjectClassType::Domain {
            
            println!(
                "{:?}",
                Domain::from(registry_record)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::{Endpoints, Registry};
    #[test]
    fn ripe() {
        let registry = Registry::connect(
            "rdap.db.ripe.net",
            Endpoints {
                domain: String::from("/domain"),
                autnum: String::from("/autnum"),
            },
        );
        registry.query_domain("193.0.6.139.in-addr.arpa");
        assert!(false);
    }
}
