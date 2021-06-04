use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use npm_resource::resource::resource::Params;
use serde_json::Value;
use tempfile::TempDir;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;

mod common;

fn prepare_out(path: &PathBuf, version: &str) {
    let package_dir = path.join("my-package");
    let version_dir = path.join("version");

    fs::create_dir_all(&package_dir).unwrap();
    fs::create_dir_all(&version_dir).unwrap();

    common::build_fake_package(&package_dir, Some(".npmignore"));
    common::write_version_file(&version_dir, "version", version);
}

fn extract_payload_tar(payload: &Value, tarname: &str, target_dir: &PathBuf) {
    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();

    let saved_archive_path = temp_path.join("archive.tgz");
    let base64_package = payload
        .pointer(&format!("/_attachments/{}/data", tarname))
        .unwrap()
        .as_str()
        .unwrap();

    let mut saved_archive = File::create(&saved_archive_path).unwrap();
    let package = base64::decode(base64_package).unwrap();
    saved_archive.write_all(&package).unwrap();

    let saved_archive = File::open(&saved_archive_path).unwrap();
    let tar = GzDecoder::new(saved_archive);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(target_dir).unwrap();
}

#[tokio::test]
async fn publish() -> anyhow::Result<()> {
    //////////////////////////////////////
    // Data and mock server preparation //
    //////////////////////////////////////
    let mut payload = common::read_payload(include_str!("data/put_payload_with_params.json"));
    let override_package_name = "my-override-name";

    // Directories and files
    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();
    let temp_str = temp_path.as_os_str().to_str().unwrap();
    let expected_tarname = format!("my_override_name-{}.tgz", "9.4.20");
    let archive_extract_dir = temp_path.join("extract");

    prepare_out(&temp_path, "9.4.20");

    // Server preparation
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/{}", override_package_name)))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    payload.source.registry = mock_server.uri();
    payload.params = Some(Params {
        version: Some(String::from("version/version")),
        package_name: Some(String::from(override_package_name)),
        package: Some(String::from("my-package")),
    });

    ///////////////////////////////////////////////
    // Binary invocation - publish the given dir //
    ///////////////////////////////////////////////
    let result = common::run("out", &serde_json::to_string(&payload)?, temp_str);
    let _output = common::read_json(&result);

    ////////////////////////////
    // Check received payload //
    ////////////////////////////
    let received_requests = mock_server.received_requests().await.unwrap();
    let publish_request = received_requests.first().unwrap();

    let publish_body: Value = serde_json::from_slice(&publish_request.body).unwrap();

    extract_payload_tar(&publish_body, &expected_tarname, &archive_extract_dir);

    assert!(&archive_extract_dir.join("package").is_dir());
    assert!(&archive_extract_dir.join("package/package.json").is_file());
    assert!(!&archive_extract_dir.join("package/file-to-ignore").exists());

    Ok(())
}

#[tokio::test]
async fn publish_test_light() -> anyhow::Result<()> {
    //////////////////////////////////////
    // Data and mock server preparation //
    //////////////////////////////////////
    let mut payload = common::read_payload(include_str!("data/put_payload_light.json"));
    let override_package_name = "my-override-name";

    // Directories and files
    let temp = TempDir::new().unwrap();
    let temp_path = temp.into_path();
    let temp_str = temp_path.as_os_str().to_str().unwrap();
    let expected_tarname = format!("my_override_name-{}.tgz", "9.4.20");
    let archive_extract_dir = temp_path.join("extract");

    prepare_out(&temp_path, "9.4.20");

    // Server preparation
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(format!("/{}", override_package_name)))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    eprintln!("listening on {}", mock_server.uri());
    payload.source.registry = mock_server.uri();
    payload.params = Some(Params {
        version: Some(String::from("version/version")),
        package_name: Some(String::from(override_package_name)),
        package: Some(String::from("my-package")),
    });

    ///////////////////////////////////////////////
    // Binary invocation - publish the given dir //
    ///////////////////////////////////////////////
    let result = common::run("out", &serde_json::to_string(&payload)?, temp_str);
    let _output = common::read_json(&result);

    ////////////////////////////
    // Check received payload //
    ////////////////////////////
    let received_requests = mock_server.received_requests().await.unwrap();
    let publish_request = received_requests.first().unwrap();

    let publish_body: Value = serde_json::from_slice(&publish_request.body).unwrap();

    extract_payload_tar(&publish_body, &expected_tarname, &archive_extract_dir);

    assert!(&archive_extract_dir.join("package").is_dir());
    assert!(&archive_extract_dir.join("package/package.json").is_file());
    assert!(!&archive_extract_dir.join("package/file-to-ignore").exists());

    Ok(())
}
