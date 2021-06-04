use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::npm::registry_client::RegistryClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub version: Option<CheckVersion>,
    pub source: Source,
    pub params: Option<Params>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(default = "default_registry")]
    pub registry: String,
    pub package_name: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    #[serde(default = "default_auth_type")]
    pub auth_type: String,
    #[serde(default = "default_tag")]
    pub tag: String,
}

impl Source {
    pub async fn check(
        &self,
        registry: &RegistryClient,
        version: &Option<CheckVersion>,
    ) -> anyhow::Result<Vec<CheckVersion>> {
        let package = registry.fetch_package_manifest(&self.package_name).await?;

        let versions: Vec<CheckVersion>;

        if &self.tag == "semver" {
            // let current_version = &version;
            versions = match version {
                None => {
                    let sorted_versions = package.get_sorted_versions();
                    let last_version = sorted_versions.last().expect("No version found");
                    vec![CheckVersion::from_semver(*last_version)]
                }
                Some(version) => package
                    .get_version_after(&version.version)
                    .iter()
                    .map(|&sv| CheckVersion::from_semver(sv))
                    .collect::<Vec<CheckVersion>>(),
            };
        } else {
            versions = match package.get_tag_version(&self.tag) {
                Some(version) => vec![CheckVersion::from_semver(version)],
                None => vec![],
            };
        }
        Ok(versions)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub version: Option<String>,
    pub package_name: Option<String>,
    pub package: Option<String>,
}

impl Params {
    /// Extract the param version field, if version is a semver value
    /// it's returned as a String else we assume it's a relative path to a file
    /// containing a version
    /// ```
    /// use tempfile::tempdir;
    /// use std::fs::File;
    /// use std::io::{self, Write};
    /// use npm_resource::resource::resource::Params;
    ///
    /// let mut p = Params {version: Some("1.0.0".to_string()), package_name: None, package: None};
    /// assert_eq!(p.extract_version(&std::env::temp_dir()).unwrap(), "1.0.0");
    /// p.version = Some("1.0.0-aaa".to_string());
    /// assert_eq!(p.extract_version(&std::env::temp_dir()).unwrap(), "1.0.0-aaa");
    ///
    /// let dir = tempdir().unwrap();
    /// let file_path = dir.path().join("version");
    /// let mut file = File::create(file_path).unwrap();
    /// writeln!(file, "1.0.2").unwrap();
    /// p.version = Some("version".to_string());
    /// assert_eq!(p.extract_version(&dir.into_path()).unwrap(), "1.0.2");
    ///
    /// ```
    pub fn extract_version(&self, put_folder: &PathBuf) -> Option<String> {
        let version = self.version.to_owned();
        version.map(|key| match semver::Version::parse(&key) {
            Ok(version) => version.to_string(),
            Err(_) => {
                let file_path = put_folder.join(key);
                let new_version =
                    fs::read_to_string(file_path).expect("Could not read version file");
                String::from(new_version.trim_end_matches("\n"))
            }
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CheckVersion {
    pub version: semver::Version,
}

impl CheckVersion {
    pub fn new(version: &str) -> anyhow::Result<CheckVersion> {
        let version = Version::parse(version)?;
        Ok(CheckVersion { version: version })
    }

    pub fn from_semver(ver: &semver::Version) -> CheckVersion {
        CheckVersion {
            version: ver.to_owned(),
        }
    }
}

fn default_registry() -> String {
    String::from("https://registry.npmjs.org/")
}

fn default_auth_type() -> String {
    String::from("anonymous")
}

fn default_tag() -> String {
    String::from("latest")
}
