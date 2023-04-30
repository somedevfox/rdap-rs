use crate::model;

pub struct Endpoints {
    pub domain: String,
    pub autnum: String,
}
pub struct Registry {
    domain: String,
    endpoints: Endpoints
}
impl Registry {
    pub fn get(domain: impl Into<String>, endpoints: Endpoints) -> Self {
        Self {
            domain: domain.into(),
            endpoints
        }
    }

    pub fn query_domain(&self, domain_name: impl ToString) {
        println!("{:?}", ureq::get(format!("https://{}{}/{}", self.domain, self.endpoints.domain, domain_name.to_string()).as_str())
            .call()
            .unwrap()
            .into_json::<model::RegistryRecord>()
            .unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::{Endpoints, Registry};
    #[test]
    fn ripe() {
        let registry = Registry::get("rdap.db.ripe.net", Endpoints { domain: String::from("/domain"), autnum: String::from("/autnum") });
        registry.query_domain("193.0.6.139.in-addr.arpa");
        assert!(false);
    }
}