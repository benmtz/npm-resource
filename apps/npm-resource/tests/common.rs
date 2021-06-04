use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use npm_resource::resource::resource::Payload;
use serde_json::Value;

#[cfg(test)]
#[allow(dead_code)]
pub fn get_binary(name: &str) -> String {
    let bin_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_dir = bin_dir.join(format!("target/debug/{}", name));
    bin_dir.as_os_str().to_str().unwrap().to_owned()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn run(binary: &str, stdin_str: &str, file_arg: &str) -> String {
    let mut child = Command::new(get_binary(binary))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(file_arg)
        .spawn()
        .unwrap();
    let child_stdin = child.stdin.as_mut().unwrap();
    child_stdin.write_all(stdin_str.as_bytes()).unwrap();
    drop(child_stdin);
    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn read_payload(content: &str) -> Payload {
    serde_json::from_str::<Payload>(content).unwrap()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn read_json(content: &str) -> Value {
    serde_json::from_str(content).unwrap()
}

#[cfg(test)]
#[allow(dead_code)]
pub fn semver_eq(ver: &semver::Version, str: &str) -> () {
    let parsed = semver::Version::parse(str).unwrap();
    assert_eq!(ver, &parsed);
}

#[cfg(test)]
#[allow(dead_code)]
pub fn build_fake_package(dir: &PathBuf, ignore_file: Option<&str>) -> () {
    let mut package = File::create(dir.join("package.json")).unwrap();
    package
        .write_all(include_bytes!("data/package/source/package.json"))
        .unwrap();

    let mut avatar = File::create(dir.join("avatar.png")).unwrap();
    avatar
        .write_all(include_bytes!("data/package/source/avatar.png"))
        .unwrap();

    let mut readme = File::create(dir.join("README.md")).unwrap();
    readme
        .write_all(include_bytes!("data/package/source/README.md"))
        .unwrap();

    let mut ignore = File::create(dir.join("file-to-ignore")).unwrap();
    ignore
        .write_all(include_bytes!("data/package/source/file-to-ignore"))
        .unwrap();

    if let Some(ignore_filename) = ignore_file {
        let mut ignore_file = File::create(dir.join(ignore_filename)).unwrap();
        ignore_file
            .write_all(include_bytes!("data/package/source/ignore"))
            .unwrap();
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub fn write_version_file(version_dir: &PathBuf, file_name: &str, version: &str) -> () {
    let mut version_file = File::create(version_dir.join(file_name)).unwrap();
    version_file.write_all(version.as_bytes()).unwrap();
}
