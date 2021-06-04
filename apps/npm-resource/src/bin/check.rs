use std::io;
use std::io::Read;

use npm_resource::npm::registry_client::RegistryClient;
use npm_resource::resource::resource::Payload;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Concourse input parsing
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let payload = serde_json::from_str::<Payload>(&buffer).expect("Invalid payload");

    // Registry Client Building
    let registry_client = RegistryClient::from_source(&payload.source).await;

    let versions = &payload
        .source
        .check(&registry_client, &payload.version)
        .await?;
    println!("{}", serde_json::to_string(&versions).unwrap());

    Ok(())
}
