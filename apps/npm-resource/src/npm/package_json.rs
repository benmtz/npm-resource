use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::integrity;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;
use tempfile::TempDir;

pub struct PackageJson {
    pub value: Value,
    pub name: String,
    pub version: String,
}

impl PackageJson {
    pub fn from_file(path: &PathBuf) -> PackageJson {
        let package_json: Value = fs::read_to_string(path)
            .map(|content| serde_json::from_str(&content).expect("Malformed package.json"))
            .expect("Could not open package.json");

        let package_name = package_json
            .get("name")
            .expect("No name found in package.json")
            .as_str()
            .expect("Could not parse package name")
            .to_owned();

        let package_version = package_json
            .get("version")
            .expect("No version found in packge.json ")
            .as_str()
            .expect("Could not parse package version")
            .to_owned();

        PackageJson {
            value: package_json,
            name: package_name,
            version: package_version,
        }
    }

    pub fn into_file(&self, path: &PathBuf) -> Result<()> {
        let tmp = TempDir::new().unwrap();
        let tmp_pkg_path = tmp.path().join("package.json");
        let mut file = File::create(&tmp_pkg_path).expect("Could not create new package.json");
        file.write_all(
            serde_json::to_string(&self.value)
                .expect("Could not serialize back package.json after overriding")
                .as_bytes(),
        )
        .expect("Could not write data to package.json");

        fs::copy(tmp_pkg_path, path).expect("Could not replace old package.json");
        Ok(())
    }

    pub fn attach_dist(&mut self, url: String, archive: &PathBuf) -> anyhow::Result<()> {
        let mut file_content: Vec<u8> = vec![];
        let mut file = File::open(&archive)?;
        file.read_to_end(&mut file_content)?;

        let pkg = self.value.as_object_mut().unwrap();
        pkg.insert(
            String::from("dist"),
            json!({
                "integrity": integrity::ssri_512(&file_content),
                "shasum": integrity::shasum(&file_content),
                "tarball": url
            }),
        );
        Ok(())
    }
}
