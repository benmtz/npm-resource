use std::{
    env,
    io::{self, Read},
};

use npm_resource::npm::registry_client::RegistryClient;
use npm_resource::resource::resource::Payload;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // A resource type receive as $1 a path and config as stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let payload = serde_json::from_str::<Payload>(&buffer).expect("Invalid payload");

    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];

    // Registry Client Building
    let registry_client = RegistryClient::from_source(&payload.source).await;
    let package_version = &payload.version.unwrap();

    registry_client
        .download(
            &payload.source.package_name,
            &package_version.version,
            target_path,
        )
        .await
        .expect("Failed to download artifact");

    println!(
        "{}",
        serde_json::to_string(&json!({ "version": &package_version })).unwrap()
    );
    Ok(())
}
