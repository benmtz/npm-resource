/// Returns bytes shasum
///
/// ```
/// let decoded = npm_resource::utils::basic_token::decode("YWFhOmJiYg==");
/// assert_eq!(decoded.0, "aaa");
/// assert_eq!(decoded.1, "bbb");
/// ```
pub fn decode(basic_token: &str) -> (String, String) {
    let decoded =
        String::from_utf8(base64::decode(basic_token).expect("Could not decode Basic token"))
            .expect("Could not convert token to utf8");
    let decoded = decoded.split_once(":").expect("Provided token isn't valid");
    (String::from(decoded.0), String::from(decoded.1))
}
