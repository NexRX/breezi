use regex::Regex;
use std::sync::LazyLock;

/// `#[validate(regex(path = *REGEX_USERNAME, code = "username"))]`
pub static REGEX_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]{1,32}$").unwrap());

/// `#[validate(regex(path = *REGEX_UUID, code = "uuid"))]`
pub static REGEX_UUID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap()
});
