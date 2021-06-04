use npm_resource::resource::resource::CheckVersion;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;

mod common;

// If no version is found corresponding the payload then an empty array
// should be returned
#[tokio::test]
async fn test_check_with_version_and_no_match() {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/base_payload.json"));

    // Server preparation
    let server = MockServer::start().await;
    payload.source.registry = server.uri();

    Mock::given(method("GET"))
        .and(path(format!("/{}", payload.source.package_name)))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(common::read_json(include_str!(
                "data/npm/acme-core-manifest.json"
            ))),
        )
        .mount(&server)
        .await;

    // Mock prep
    // Binary invocation
    let result = common::run("check", &serde_json::to_string(&payload).unwrap(), "");
    let found_versions = serde_json::from_str::<Vec<CheckVersion>>(&result).unwrap();

    //Verify
    assert_eq!(found_versions.len(), 0); // Unknown tag
}

// If a new version is found corresponding to the provided payload
// then the new version should be returned in as the sole item in an array
#[tokio::test]
async fn test_check_version_and_match() -> anyhow::Result<()> {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/base_payload.json"));
    let manifest = common::read_json(include_str!("data/npm/acme-core-manifest.json"));

    // Server preparation
    let server = MockServer::start().await;
    payload.source.registry = server.uri();
    payload.source.tag = String::from("latest");

    Mock::given(method("GET"))
        .and(path(format!("/{}", payload.source.package_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&manifest))
        .mount(&server)
        .await;

    let latest_version = manifest
        .get("dist-tags")
        .unwrap()
        .get("latest")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();

    // Binary invocation
    let result = common::run("check", &serde_json::to_string(&payload)?, "");
    let found_versions = serde_json::from_str::<Vec<CheckVersion>>(&result)?;

    //Verify
    assert_eq!(found_versions.len() as u64, 1); // latest tag
    common::semver_eq(&found_versions.get(0).unwrap().version, &latest_version);

    Ok(())
}

// If there is many versions since last checks (here between 0.0.0 and 0.0.7 there has been 6 versions)
// Return every version greater than or equal to the payload's version
#[tokio::test]
async fn test_semver_multiple_versions() -> anyhow::Result<()> {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/base_payload.json"));
    let manifest = common::read_json(include_str!("data/npm/acme-core-manifest.json"));

    // Server preparation
    let server = MockServer::start().await;
    payload.source.registry = server.uri();
    payload.source.tag = String::from("semver");

    Mock::given(method("GET"))
        .and(path(format!("/{}", payload.source.package_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&manifest))
        .mount(&server)
        .await;

    // Binary invocation
    let result = common::run("check", &serde_json::to_string(&payload)?, "");
    let found_versions = serde_json::from_str::<Vec<CheckVersion>>(&result)?;

    //Verify
    assert!(found_versions.len() == 8); // 0.0.0 is include (still a valid semver)
    common::semver_eq(&found_versions.get(0).unwrap().version, "0.0.0");
    common::semver_eq(&found_versions.last().unwrap().version, "0.0.7");

    Ok(())
}
