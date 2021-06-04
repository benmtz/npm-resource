use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, semver::Version>,
    pub versions: HashMap<semver::Version, Value>,
    pub time: HashMap<String, DateTime<Utc>>,
}

impl Metadata {
    pub fn get_tarball_url(&self, version: &semver::Version) -> Option<String> {
        self.versions
            .get(version)
            .and_then(|versions| versions.get("dist"))
            .and_then(|dist| dist.get("tarball"))
            .and_then(|v| Value::as_str(v).map(String::from))
    }

    pub fn get_sorted_versions(&self) -> Vec<&semver::Version> {
        let mut versions = self.versions.keys().collect::<Vec<&semver::Version>>();
        versions.sort_by(|&a, &b| a.cmp(b));
        return versions;
    }

    pub fn get_tag_version(&self, tag: &str) -> Option<&semver::Version> {
        self.dist_tags.get(tag)
    }

    pub fn get_version_after(&self, version: &semver::Version) -> Vec<&semver::Version> {
        let mut versions_after = vec![];
        for &v in self.get_sorted_versions().iter() {
            if v.ge(version) {
                versions_after.push(v)
            }
        }
        versions_after
    }
}
