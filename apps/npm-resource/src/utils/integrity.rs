use sha1::Sha1;
use sha2::Digest;
use sha2::Sha512;

/// Returns bytes shasum
///
/// ```
/// let shasum = npm_resource::utils::integrity::shasum(b"hello world");
/// assert_eq!(shasum, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
/// ```
pub fn shasum(body: &[u8]) -> String {
    let mut shasum_builder = Sha1::new();
    shasum_builder.update(body);
    shasum_builder.digest().to_string()
}

/// Returns the Standard Subresource Integrity with a sha512 algorithm
///
/// ```
/// let hello_world_ssri = npm_resource::utils::integrity::ssri_512(b"hello world");
/// assert_eq!(hello_world_ssri, "sha512-MJ7MSJwS1utMxA9QyQLytNDtd+5RGnx6m808qG1M2G+YndNbxf9JlnDaNCVbRbDP2DDoH2Bdz33FVC6TrpzXbw==");
/// ```
pub fn ssri_512(body: &[u8]) -> String {
    format!("sha512-{}", base64::encode(Sha512::digest(body)))
}
