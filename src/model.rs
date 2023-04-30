//! (De)serializable data models according to [RFC7483](https://datatracker.ietf.org/doc/html/rfc7483) required for the **Registration Data Access Protocol** to function.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub type Links = Vec<Link>;
pub type Events = Vec<Event>;
pub type Notices = Vec<Notice>;
pub type PublicIDs = Vec<PublicID>;

/// Representation of a web link as defined in [Section 4.2](https://datatracker.ietf.org/doc/html/rfc7483#section-4.2).
/// 
/// All but `href` members are OPTIONAL and should not be blindly unwrapped.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    pub value: Option<String>,
    pub rel: Option<String>,
    pub href: String,
    pub hreflang: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>
}

/// Representation of a singular event happened to an [Internet Protocol Address, Autonomous System, Nameserver or Entity](ObjectClassType) as defined in [Section 4.5](https://datatracker.ietf.org/doc/html/rfc7483#section-4.5).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    /// String denoting the reason for the event.
    #[serde(rename = "eventAction")]
    pub action: String,
    /// An identifier denoting the actor responsible for the event.
    #[serde(rename = "eventActor")]
    pub actor: Option<String>,
    /// String containing the time and date the event has occured.
    #[serde(rename = "eventDate")]
    pub date: String,
    /// Array of [links](Links) relevant to the event.
    pub links: Option<Links>
}

/// Object Classes denote how should a response be parsed.
/// 
/// Specification for the object classes: [Section 5](https://datatracker.ietf.org/doc/html/rfc7483#section-5)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ObjectClassType {
    Domain,
    #[serde(rename = "ip network")]
    IpNetwork,
    #[serde(rename = "autnum")]
    AutonomousSystem,
    Entity,
    Nameserver
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notice {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub notice_type: Option<String>,
    pub description: Vec<String>,
    pub links: Option<Links>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicID {
    #[serde(rename = "type")]
    pub id_type: String,
    pub identifier: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IpVersion {
    V4,
    V6
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "objectClassType")]
    pub entity_type: Option<ObjectClassType>,
    pub handle: String,
    #[serde(rename = "vcardArray")]
    pub vcard_array: Option<Vec<JsonValue>>,
    pub roles: Vec<String>,
    pub links: Option<Links>,
    pub events: Option<Events>,
    pub status: Option<Vec<String>>,
    pub port43: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nameserver {
    #[serde(rename = "ldhName")]
    pub ldh_name: String,
    #[serde(rename = "objectClassName")]
    pub nameserver_type: ObjectClassType
}

/// Representation of a Registration Data Access Protocol Response from a Regional Internet Registry.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistryRecord {
    /// An array of strings providing a hint as to the specifications used in the construction of the response.
    /// 
    /// ID "rdap_level_0" signifies conformance with [RFC7483](https://datatracker.ietf.org/doc/html/rfc7483) specification primarily used in this crate.
    /// 
    /// Materials useful for parsing RDAP Extensions:
    /// - [Section 4.1 "RDAP Conformance" of RFC7483](https://datatracker.ietf.org/doc/html/rfc7483#section-4.1)
    /// - [Section 6 "Extensibility" of RFC7480](https://datatracker.ietf.org/doc/html/rfc7480#section-6)
    /// - [Section 8.1 "RDAP Extensions Registry" of RFC7480](https://datatracker.ietf.org/doc/html/rfc7480#section-8.1)
    #[serde(rename = "rdapConformance")]
    pub rdap_conformance: Option<Vec<String>>,
    pub notices: Notices,
    pub lang: Option<String>,
    pub network: Option<Box<RegistryRecord>>,
    #[serde(rename = "objectClassName")]
    pub record_type: ObjectClassType,
    #[serde(rename = "startAddress")]
    pub start_address: Option<String>,
    #[serde(rename = "endAddress")]
    pub end_address: Option<String>,
    pub handle: String,
    #[serde(rename = "ldhName")]
    pub ldh_name: Option<String>,
    #[serde(rename = "nameServers")]
    pub nameservers: Option<Vec<Nameserver>>,
    #[serde(rename = "startAutnum")]
    pub start_autnum: Option<u32>,
    #[serde(rename = "endAutnum")]
    pub end_autnum: Option<u32>,
    #[serde(rename = "ipVersion")]
    pub ip_version: Option<IpVersion>,
    pub name: Option<String>,
    #[serde(rename = "parentHandle")]
    pub parent_handle: Option<String>,
    pub remarks: Vec<Notice>,
    pub events: Events,
    pub country: Option<String>,
    #[serde(rename = "publicIds")]
    pub public_ids: Option<PublicIDs>,
    pub entities: Vec<Entity>,
    pub port43: Option<String>,
}