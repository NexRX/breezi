use derive_more::{Deref, DerefMut};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};
use validator::{Validate, ValidationErrors};

/// `#[validate(regex(path = *REGEX_USERNAME, code = "username"))]`
pub static REGEX_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]{1,32}$").unwrap());

/// `#[validate(regex(path = *REGEX_UUID, code = "uuid"))]`
pub static REGEX_UUID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap()
});

#[allow(dead_code)] // Keep it here, it works but then you have to use try/catch in js land
#[derive(Debug, Deref, DerefMut, Clone)]
struct Validated<T: Clone>(T);

impl<T: Validate + Clone> Validated<T> {
    pub fn new(value: T) -> Result<Self, ValidationErrors> {
        value.validate()?;
        Ok(Self(value))
    }
}

impl<T: ts_rs::TS + Clone> ts_rs::TS for Validated<T> {
    type WithoutGenerics = T;

    fn decl() -> String {
        T::decl()
    }

    fn decl_concrete() -> String {
        T::decl_concrete()
    }

    fn name() -> String {
        T::name()
    }

    fn inline() -> String {
        T::inline()
    }

    fn inline_flattened() -> String {
        T::inline_flattened()
    }
}

impl<'de, T: Deserialize<'de> + Validate + Clone> Deserialize<'de> for Validated<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        println!("Deserializing Validated<T>");
        let t = T::deserialize(deserializer)?;
        let validated = Self::new(t)
            .map_err(|validation_error| serde::de::Error::custom(format!("validation error: {validation_error:?}")))?;
        Ok(validated)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
pub struct Invalidation {
    code: String,
    message: String,
    value: Value,
    rules: HashMap<String, String>,
}

impl Invalidation {
    pub fn new(code: impl ToString, message: impl ToString, value: Value, rules: impl Into<HashMap<String, String>>) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            value,
            rules: rules.into(),
        }
    }
}
