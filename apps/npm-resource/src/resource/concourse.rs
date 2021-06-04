use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use crate::npm::publish::PublishPayload;

// Making Metadata structs enables to display things in a nice looking table in concourse
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub value: Value,
}

impl Metadata {
    pub fn new(key: &str, val: Option<&Value>) -> Metadata {
        Metadata {
            name: String::from(key),
            value: val.unwrap_or(&json!("")).to_owned(),
        }
    }

    // from_publish_payload gets some usefull data from a publish payload and convert it to a Metadata array
    pub fn from_publish_payload(publish_payload: &PublishPayload) -> Vec<Metadata> {
        let mut metadata_list = vec![];
        let released_version = publish_payload.get_last_published().unwrap();

        metadata_list.push(Metadata::new("version", released_version.get("version")));
        metadata_list.push(Metadata::new("name", released_version.get("name")));
        metadata_list.push(Metadata::new(
            "integrity",
            released_version.get("dist").unwrap().get("integrity"),
        ));
        metadata_list.push(Metadata::new(
            "shasum",
            released_version.get("dist").unwrap().get("shasum"),
        ));
        metadata_list.push(Metadata::new(
            "tarball",
            released_version.get("dist").unwrap().get("tarball"),
        ));

        metadata_list
    }
}
