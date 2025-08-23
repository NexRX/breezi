use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

use crate::logic::Invalidation;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ts_rs::TS)]
pub struct ErrorResponse {
    pub reason: ErrorReason,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    pub const INTERNAL_MESSAGE: &str = "Internal server error";

    pub fn new(reason: ErrorReason, message: String) -> ErrorResponse {
        ErrorResponse {
            reason,
            message,
            timestamp: Utc::now(),
        }
    }

    pub fn internal() -> ErrorResponse {
        Self::default()
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self::new(ErrorReason::Internal, Self::INTERNAL_MESSAGE.to_string())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, ts_rs::TS, derive_more::IsVariant)]
pub enum ErrorReason {
    BadRequest,
    Invalid(HashMap<String, Invalidation>),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    #[default]
    Internal,
    ServiceUnavailable,
    GatewayTimeout,
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Protocol(_) => ErrorResponse::new(ErrorReason::BadRequest, "Protocol error - Bad Request".into()),
            sqlx::Error::ColumnIndexOutOfBounds { .. } => ErrorResponse::new(
                ErrorReason::BadRequest,
                "Column index out of bounds - Bad Request".to_string(),
            ),
            sqlx::Error::ColumnNotFound(_) => {
                ErrorResponse::new(ErrorReason::BadRequest, "Column not found - Bad Request".to_string())
            }
            sqlx::Error::ColumnDecode { .. } => {
                ErrorResponse::new(ErrorReason::BadRequest, "Column decode error - Bad Request".to_string())
            }
            sqlx::Error::Decode(_) => ErrorResponse::new(ErrorReason::BadRequest, "Decode error - Bad Request".to_string()),
            sqlx::Error::RowNotFound => ErrorResponse::new(ErrorReason::NotFound, "Entry not found".to_string()),
            sqlx::Error::PoolClosed => ErrorResponse::new(
                ErrorReason::ServiceUnavailable,
                "Database pool closed - Service Unavailable".to_string(),
            ),
            sqlx::Error::Io(_) => ErrorResponse::new(
                ErrorReason::ServiceUnavailable,
                "Database I/O error - Service Unavailable".to_string(),
            ),
            sqlx::Error::Tls(_) => ErrorResponse::new(
                ErrorReason::ServiceUnavailable,
                "Database TLS error - Service Unavailable".to_string(),
            ),
            sqlx::Error::PoolTimedOut => ErrorResponse::new(
                ErrorReason::GatewayTimeout,
                "Database pool timed out (504 Gateway Timeout)".to_string(),
            ),
            _ => ErrorResponse::default(),
        }
    }
}
impl From<ValidationErrors> for ErrorResponse {
    fn from(value: ValidationErrors) -> Self {
        let mut messages = Vec::new();

        for (field, errors) in value.field_errors().iter() {
            for error in errors.iter() {
                let msg = error
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid field '{}': {}", field, error.code));
                messages.push(msg);
            }
        }

        // Sort messages for consistent output
        messages.sort();

        let reasons: HashMap<String, Invalidation> = value
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                // Take the first error for this field
                let error = &errors[0];

                let code = if error.code == "regex" {
                    "pattern match".to_string()
                } else {
                    error.code.to_string()
                };

                // Collect and sort rules for consistent output
                let rules: HashMap<String, String> = error
                    .params
                    .iter()
                    .filter(|(k, _)| *k != "value")
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();

                let value = error.params.get("value").cloned().unwrap_or(serde_json::Value::Null);
                let message = error
                    .message
                    .as_ref()
                    .map(|msg| msg.to_string())
                    .unwrap_or_else(|| format!("Given value is not a valid {code}"));

                (field.to_string(), Invalidation::new(code, message, value, rules))
            })
            .collect();

        ErrorResponse::new(ErrorReason::Invalid(reasons), "One or more fields are invalid".into())
    }
}

macro_rules! impl_from_report_for_error_response {
    ($($err_ty:ty),*) => {
        impl From<color_eyre::Report> for ErrorResponse {
            fn from(value: color_eyre::Report) -> Self {
                $(
                    if value.is::<$err_ty>() {
                        return value.downcast::<$err_ty>().unwrap().into();
                    }
                )*
                Self::internal()
            }
        }
    };
}
impl_from_report_for_error_response!(sqlx::Error, ValidationErrors);
