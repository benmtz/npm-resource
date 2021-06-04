/// Strips unwanted characters from package names
/// ```
/// use npm_resource::npm::package_name;
///
/// assert_eq!(package_name::normalize_for_tarball("@acme/core"), "acme_core");
/// assert_eq!(package_name::normalize_for_tarball("@acme/rest-client"), "acme_rest_client");
/// ```
pub fn normalize_for_tarball(package_name: &str) -> String {
    package_name
        .replace("@", "")
        .replace("-", "_")
        .replace("/", "_")
        .to_ascii_lowercase()
}

/// Url encode / in package
/// ```
/// use npm_resource::npm::package_name;
///
/// assert_eq!(package_name::normalize_for_api("@acme/core"), "@acme%2fcore");
/// assert_eq!(package_name::normalize_for_api("@acme/rest-client"), "@acme%2frest-client");
/// ```
pub fn normalize_for_api(package_name: &str) -> String {
    package_name.replace("/", "%2f")
}
