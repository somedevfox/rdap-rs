pub mod model;
pub mod sync;
pub mod result;

use std::net::IpAddr;
use std::str::FromStr;

use model::Entity;
pub use model::IpVersion;
pub use model::Link;
pub use model::Event;
use model::ObjectClassType;

#[derive(Debug, Clone)]
pub struct Country {
    pub code: String,
    pub name: String,
}
impl Country {
    pub fn from_alpha2_code(code: impl Into<String>) -> Self {
        let code = code.into();
        Self {
            code: code.clone(),
            name: iso3166_1::alpha2(code.as_str()).unwrap().name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    pub start_address: IpAddr,
    pub end_address: IpAddr,
    pub ip_version: IpVersion,
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub country: Country,
    pub network: Network,
    pub nameservers: Vec<String>,
    pub events: Vec<Event>,
    pub entities: Vec<Entity>,
    pub rir_domain: String
}
impl From<model::RegistryRecord> for Domain {
    fn from(registry_record: model::RegistryRecord) -> Self {
        let network = registry_record.network.unwrap();
        Self {
            name: registry_record.handle,
            country: Country::from_alpha2_code(network.country.unwrap()),
            network: Network {
                start_address: IpAddr::from_str(&network.start_address.unwrap().to_string())
                    .unwrap(),
                end_address: IpAddr::from_str(&network.end_address.unwrap().to_string()).unwrap(),
                ip_version: network.ip_version.unwrap(),
                name: network.name.unwrap()
            },
            nameservers: registry_record
                .nameservers
                .unwrap()
                .into_iter()
                .filter(|ns| ns.nameserver_type == ObjectClassType::Nameserver)
                .map(|ns| ns.ldh_name)
                .collect(),
            events: registry_record.events,
            entities: registry_record.entities,
            rir_domain: registry_record.port43.unwrap()
        }
    }
}
