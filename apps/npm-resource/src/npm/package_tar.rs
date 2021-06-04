use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Value;
use tempfile::Builder;

use crate::npm::package_json::PackageJson;
use crate::npm::package_name;
use crate::npm::publish::PublishPayload;
use crate::npm::registry_client::RegistryClient;
use crate::utils::tar as tar_utils;

// PackageTar is an aggregation of a tarfile and its associated package.json
pub struct PackageTar {
    archive_path: PathBuf,
    package: PackageJson,
}

impl PackageTar {
    pub fn from_dir(registry: &RegistryClient, dir_path: &PathBuf) -> anyhow::Result<PackageTar> {
        let tmp = Builder::new().prefix("npm_resource").tempdir()?;
        let archive_path = tmp.path().join("temp-package.tgz");
        let output = dir_path.join("temp-package.tgz");

        tar_utils::pack(&dir_path, &archive_path)?;
        fs::copy(archive_path, &output)?;

        PackageTar::from_file(registry, &output)
    }

    pub fn from_file(
        registry: &RegistryClient,
        archive_path: &PathBuf,
    ) -> anyhow::Result<PackageTar> {
        let archive_name = archive_path.file_name().unwrap().to_str().unwrap();

        let temp_dir = Builder::new().prefix("npm_resource").tempdir()?;
        let working_dir = temp_dir.path();
        let package_path = &working_dir.join("package.json");

        // We extract tarball to read it's content
        tar_utils::unpack(archive_path, working_dir.as_os_str().to_str().unwrap())?;
        let mut pkg = PackageJson::from_file(&package_path);
        pkg.attach_dist(
            registry.build_tar_url(&pkg.name, archive_name),
            archive_path,
        )?;

        Ok(PackageTar {
            archive_path: archive_path.to_owned(),
            package: pkg,
        })
    }

    pub fn from(registry: &RegistryClient, archive_path: &PathBuf) -> anyhow::Result<PackageTar> {
        if archive_path.is_dir() {
            PackageTar::from_dir(registry, archive_path)
        } else {
            PackageTar::from_file(registry, archive_path)
        }
    }

    pub fn into_publish_payload(&self, tag: &str) -> anyhow::Result<PublishPayload> {
        let current_version = self.package.version.to_owned();
        let current_name = self.package.name.to_owned();

        let mut versions: HashMap<String, Value> = HashMap::new();
        let mut dist_tags: HashMap<String, String> = HashMap::new();

        dist_tags.insert(tag.to_string(), self.package.version.to_string());
        versions.insert(
            self.package.version.to_string(),
            self.package.value.to_owned(),
        );

        let mut payload = PublishPayload {
            _last_version: String::from(current_version),
            id: current_name.to_owned(),
            name: current_name.to_owned(),
            description: None, // TODO parse description
            dist_tags: dist_tags,
            versions: versions,
            readme: None, // TODO parse readme
            attachments: HashMap::new(),
        };

        payload.attach_dist(&self.archive_path)?;

        Ok(payload)
    }

    pub fn override_package(
        &self,
        registry: &RegistryClient,
        values: HashMap<String, String>,
        output_dir: &Path,
    ) -> anyhow::Result<PackageTar> {
        // Paths preparation
        let package_path = &output_dir.join("package");
        let new_package_json = &output_dir.join("tmp-package.json");
        let original_package_json = &package_path.join("package.json");

        let _ = fs::create_dir_all(&package_path);
        tar_utils::unpack(
            &self.archive_path,
            package_path.as_os_str().to_str().unwrap(),
        )?;

        // Package data extraction
        let mut package = PackageTar::read_package_json(&original_package_json)?;

        for (key, value) in values {
            package.insert(String::from(key), json!(value));
        }

        let new_tar_path = &output_dir.join(format!(
            "{}-{}.tgz",
            package_name::normalize_for_tarball(package.get("name").unwrap().as_str().unwrap()),
            package.get("version").unwrap().as_str().unwrap(),
        ));

        // Original package replacement
        let mut file = File::create(&new_package_json).expect("Could not create new package.json");
        file.write_all(
            serde_json::to_string(&package)
                .expect("Could not serialize back package.json after overriding")
                .as_bytes(),
        )
        .expect("Could not write data to package.json");

        fs::copy(new_package_json, original_package_json)
            .expect("Could not replace old package.json");

        tar_utils::pack(package_path, new_tar_path)?;
        tar_utils::log_content(new_tar_path)?;

        PackageTar::from(registry, new_tar_path)
    }

    fn read_package_json(package: &PathBuf) -> anyhow::Result<HashMap<String, Value>> {
        let package_json = fs::read_to_string(&package).map(|content| {
            serde_json::from_str::<HashMap<String, Value>>(&content)
                .expect("Malformed package.json")
        })?;
        Ok(package_json)
    }
}
