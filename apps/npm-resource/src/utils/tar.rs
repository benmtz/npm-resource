use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use ignore::WalkBuilder;
use std::str::FromStr;

/// Logs a tarball entries
pub fn log_content(path: &PathBuf) -> anyhow::Result<()> {
    eprintln!("📦 {}\n===", path.display());
    let tar_gz = fs::File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    for entry in archive.entries().unwrap() {
        let encoded_entry = entry.unwrap();
        let encoded_entry_path = &encoded_entry.path()?;
        eprintln!(
            "{:30}{}B",
            encoded_entry_path.display(),
            encoded_entry.header().size()?
        );
    }
    Ok(())
}

/// Extract a tarball, stripping package in path (if present)
pub fn unpack(from: &PathBuf, to: &str) -> anyhow::Result<()> {
    let tar_gz = fs::File::open(from)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    let path_to = PathBuf::from_str(to).unwrap();
    for entry in archive.entries().unwrap() {
        let mut encoded_entry = entry.unwrap();
        let encoded_entry_path = &encoded_entry.path()?;
        let stripped_path = encoded_entry_path.strip_prefix("package")?.to_owned();
        let expected_path = path_to.join(stripped_path);
        let parent = expected_path.parent().unwrap();
        if expected_path.is_dir() {
            fs::create_dir_all(&expected_path)?;
        }
        if !parent.is_dir() {
            fs::create_dir_all(&parent)?;
        }

        encoded_entry
            .unpack(&expected_path)
            .expect("Could not unpack entry"); // &path_to.join(&path))?;
    }
    Ok(())
}

/// Pack a folder, writing content in a package subdirectory and ignoring
/// .npmignored files
pub fn pack(dir: &PathBuf, archive: &PathBuf) -> anyhow::Result<()> {
    let tar_gz = File::create(&archive)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    let walker = WalkBuilder::new(dir)
        .add_custom_ignore_filename(".npmignore")
        .build();
    for result in walker {
        let entry = result?;
        let relative_file_path = entry.path().strip_prefix(dir).unwrap();
        let target_dir = Path::new("package");
        let target_file = target_dir.join(relative_file_path);
        tar.append_path_with_name(&entry.path(), target_file)?;
    }

    tar.into_inner()?;

    Ok(())
}
