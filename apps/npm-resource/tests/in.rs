use tempfile::TempDir;

use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;

mod common;

// In should download the tar corresponding to a specific version and extract it in the given directory (as $1)
// npm package content are put in a "package" folder inside the tar, this package folder should be stripped
// in should return the downloaded version
#[tokio::test]
async fn download_and_extract() -> anyhow::Result<()> {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/v7_latest_payload.json"));

    let mut manifest = common::read_json(include_str!("data/npm/acme-core-manifest.json"));
    let artifact: &[u8] = include_bytes!("data/package/package.tgz");
    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();
    let temp_str = temp_path.as_os_str().to_str().unwrap();

    let server = MockServer::start().await;
    payload.source.registry = server.uri();
    payload.source.tag = String::from("latest");

    *manifest
        .pointer_mut("/versions/0.0.7/dist/tarball")
        .unwrap() = format!("{}/package.tgz", server.uri()).into();

    Mock::given(method("GET"))
        .and(path(format!("/{}", payload.source.package_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&manifest))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/package.tgz"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(artifact))
        .mount(&server)
        .await;

    // Binary invocation
    let result = common::run("in", &serde_json::to_string(&payload)?, temp_str);
    let output = common::read_json(&result);

    eprintln!("{:#?}", output);

    assert_eq!(
        output
            .pointer("/version/version")
            .unwrap()
            .as_str()
            .unwrap(),
        "0.0.7"
    );

    //Verify
    assert!(temp_path.join("package.json").is_file());

    Ok(())
}

// Should work without token for public registries
#[tokio::test]
async fn download_public() -> anyhow::Result<()> {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/v7_latest_public_payload.json"));

    let mut manifest = common::read_json(include_str!("data/npm/acme-core-manifest.json"));
    let artifact: &[u8] = include_bytes!("data/package/package.tgz");
    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();
    let temp_str = temp_path.as_os_str().to_str().unwrap();

    let server = MockServer::start().await;
    payload.source.registry = server.uri();
    payload.source.tag = String::from("latest");

    *manifest
        .pointer_mut("/versions/0.0.7/dist/tarball")
        .unwrap() = format!("{}/package.tgz", server.uri()).into();

    Mock::given(method("GET"))
        .and(path(format!("/{}", payload.source.package_name)))
        .respond_with(ResponseTemplate::new(200).set_body_json(&manifest))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/package.tgz"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(artifact))
        .mount(&server)
        .await;

    // Binary invocation
    let result = common::run("in", &serde_json::to_string(&payload)?, temp_str);
    let output = common::read_json(&result);

    eprintln!("{:#?}", output);

    assert_eq!(
        output
            .pointer("/version/version")
            .unwrap()
            .as_str()
            .unwrap(),
        "0.0.7"
    );

    //Verify
    assert!(temp_path.join("package.json").is_file());

    Ok(())
}

// Should work without token for public registries
#[tokio::test]
async fn real_download_public() -> anyhow::Result<()> {
    // Input data prep
    let mut payload = common::read_payload(include_str!("data/lodash_payload.json"));

    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();
    let temp_str = temp_path.as_os_str().to_str().unwrap();

    payload.source.tag = String::from("latest");

    // Binary invocation
    let result = common::run("in", &serde_json::to_string(&payload)?, temp_str);
    let output = common::read_json(&result);

    eprintln!("{:#?}", output);

    //Verify
    assert!(temp_path.join("package.json").is_file());

    Ok(())
}
