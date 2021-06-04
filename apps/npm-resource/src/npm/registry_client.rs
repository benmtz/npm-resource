use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use tempfile::Builder;
use tempfile::TempDir;

use crate::npm::npm::Metadata;
use crate::npm::package_name;
use crate::npm::publish::PublishPayload;
use crate::resource::resource::Source;
use crate::utils::{basic_token, tar};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

pub struct RegistryClient {
    url: String,
    token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryLoginResponse {
    token: String,
}

impl RegistryClient {
    pub async fn from_source(source: &Source) -> RegistryClient {
        let token = match source.auth_type.as_str() {
            "basic" => {
                let decoded_token = basic_token::decode(
                    &source
                        .token
                        .as_ref()
                        .expect("No token provided for basic auth"),
                );
                Some(
                    RegistryClient::login(&source.registry, &decoded_token.0, &decoded_token.1)
                        .await,
                )
            }
            "password" => Some(
                RegistryClient::login(
                    &source.registry,
                    &source.username.as_ref().expect("No username in source"),
                    &source.password.as_ref().expect("No password in source"),
                )
                .await,
            ),
            "bearer" => Some(String::from(
                source
                    .token
                    .as_ref()
                    .expect("No token provided for bearer auth"),
            )),
            "anonymous" => None,
            _ => panic!("Invalid auth type (must be basic|password|bearer|anonymous)"),
        };

        RegistryClient {
            url: String::from(&source.registry),
            token,
        }
    }

    pub async fn login(registry: &str, username: &str, password: &str) -> String {
        let mut map = HashMap::new();
        map.insert("name", username);
        map.insert("password", password);

        let login_client = reqwest::Client::new();

        login_client
            .put(&format!(
                "{}/-/user/org.couchdb.user:{}/-rev/undefined",
                registry, username
            ))
            .basic_auth(&username, Some(password))
            .json(&map)
            .send()
            .await
            .expect("Login failed")
            .json::<RegistryLoginResponse>()
            .await
            .expect("Login response parsing failed")
            .token
    }

    pub async fn fetch_package_manifest(&self, package_name: &str) -> anyhow::Result<Metadata> {
        let package_endpoint = self.build_package_url(package_name);
        let client = self.get_client();
        Ok(client
            .get(&package_endpoint)
            .send()
            .await?
            .error_for_status()?
            .json::<Metadata>()
            .await?)
    }

    pub async fn download(
        &self,
        package_name: &str,
        version: &semver::Version,
        target: &str,
    ) -> anyhow::Result<()> {
        let tmp_dir = Builder::new().prefix("npm_resource").tempdir()?;
        let package = self
            .fetch_package_manifest(package_name)
            .await
            .expect("Could not fetch package metadata");

        let tarball_url = &package
            .get_tarball_url(version)
            .expect("No tarball url found on package");

        let downloaded_tar = self.download_tar(&tarball_url, &tmp_dir).await?;

        tar::unpack(&downloaded_tar, &target)
    }

    pub async fn upload(
        &self,
        publish_payload: &PublishPayload,
    ) -> anyhow::Result<semver::Version> {
        let package_resource_url = self.build_package_url(&publish_payload.name);

        let client = self.get_client();
        let request = client.put(&package_resource_url);

        let _result = request
            .json(publish_payload)
            .send()
            .await?
            .error_for_status()?;

        let published_version = &publish_payload._last_version;

        Ok(semver::Version::parse(&published_version).unwrap())
    }

    pub async fn tag_version(
        &self,
        package_name: &str,
        tag: &str,
        version: &str,
    ) -> anyhow::Result<()> {
        eprintln!("version is {}", version);
        let tag_url = self.build_tag_url(package_name);
        let mut merge_tag = HashMap::new();
        merge_tag.insert(tag, version);

        let request = self.get_client().post(&tag_url).json(&merge_tag);
        let response = request.send().await?;
        response.error_for_status()?;
        Ok(())
    }

    pub fn build_package_url(&self, package_name: &str) -> String {
        format!("{}/{}", self.url, package_name)
    }

    pub fn build_tar_url(&self, package_name: &str, archive_name: &str) -> String {
        format!("{}/{}/-/{}", &self.url, package_name, archive_name)
    }

    fn build_tag_url(&self, package_name: &str) -> String {
        format!(
            "{}/-/package/{}/dist-tags",
            self.url,
            package_name::normalize_for_api(package_name)
        )
    }

    async fn download_tar(
        &self,
        tarball_url: &str,
        target_dir: &TempDir,
    ) -> anyhow::Result<PathBuf> {
        let client = self.get_client();
        let response = client.get(tarball_url).send().await?.bytes().await?;
        let tmp_target = target_dir.path().join("artifact.tgz");
        let mut dest = File::create(&tmp_target).expect("Could not create temp dir");
        dest.write_all(&response)?;
        Ok(tmp_target.to_owned())
    }

    fn get_client(&self) -> Client {
        let client = reqwest::Client::builder();

        let mut headers = header::HeaderMap::new();

        match &self.token {
            Some(token) => {
                headers.insert(
                    "Authorization",
                    header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
                );
            }
            None => (),
        };

        client
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client")
    }
}
