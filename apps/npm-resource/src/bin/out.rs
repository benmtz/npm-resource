use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use npm_resource::npm::package_tar::PackageTar;
use npm_resource::npm::registry_client::RegistryClient;
use npm_resource::resource::concourse::Metadata;
use npm_resource::resource::resource::Payload;
use serde_json::json;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // A resource type receive as $1 a path and config as stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let payload = serde_json::from_str::<Payload>(&buffer).expect("Invalid payload");

    let args: Vec<String> = env::args().collect();

    let put_folder = &args[1];
    let put_folder = PathBuf::from_str(put_folder)?;

    let registry_client = RegistryClient::from_source(&payload.source).await;
    let params = payload.params.expect("no params provided");

    let version_override_value = params.extract_version(&put_folder);

    let new_version: semver::Version;
    let response: String;

    if let Some(package_path_str) = params.package {
        let mut package_overrides = HashMap::new();
        if let Some(override_version) = &version_override_value {
            package_overrides.insert(String::from("version"), String::from(override_version));
        }

        if let Some(override_name) = params.package_name {
            package_overrides.insert(String::from("name"), String::from(override_name));
        }

        let package_path = put_folder.join(package_path_str);
        let built_package_dir = TempDir::new()?;
        let built_package_path = built_package_dir.path();

        let package_tar = PackageTar::from(&registry_client, &package_path)?.override_package(
            &registry_client,
            package_overrides,
            built_package_path,
        )?;
        let publish_payload = package_tar.into_publish_payload(&payload.source.tag)?;

        new_version = registry_client
            .upload(&publish_payload)
            .await
            .expect("Failed to upload artifact");

        response = serde_json::to_string(&json!({
            "version": {"version": new_version},
            "metadata": Metadata::from_publish_payload(&publish_payload)
        }))
        .unwrap()
    } else {
        let version = &version_override_value.expect("A version is needed for tagging");
        registry_client
            .tag_version(&payload.source.package_name, &payload.source.tag, version)
            .await?;
        response = serde_json::to_string(&json!({
            "version": {"version": version}
        }))
        .unwrap()
    }

    println!("{}", response);
    Ok(())
}
