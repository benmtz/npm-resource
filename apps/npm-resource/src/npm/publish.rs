use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishPayload {
    #[serde(skip)]
    pub _last_version: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(rename = "_attachments")]
    pub attachments: HashMap<String, Attachment>,
}

impl PublishPayload {
    pub fn get_package_version(&self, version: &str) -> anyhow::Result<&Value> {
        Ok(self.versions.get(version).unwrap())
    }
    pub fn get_last_published(&self) -> anyhow::Result<&Value> {
        self.get_package_version(&self._last_version)
    }

    pub fn attach_dist(&mut self, archive: &PathBuf) -> anyhow::Result<()> {
        let mut file_content: Vec<u8> = vec![];
        let mut file = File::open(&archive)?;
        file.read_to_end(&mut file_content)?;

        self.attachments.insert(
            archive.file_name().unwrap().to_str().unwrap().to_owned(),
            Attachment {
                content_type: String::from("application/octet-stream"),
                data: base64::encode(&file_content),
                length: file_content.len(),
            },
        );

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub content_type: String,
    pub data: String, //base64 data
    pub length: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
}
