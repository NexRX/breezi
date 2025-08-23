use serde::{Deserialize, Serialize};
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ts_rs::TS)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

impl ErrorResponse {
    pub const INTERNAL_MESSAGE: &str = "Internal server error";

    pub fn internal() -> ErrorResponse {
        Self::default()
    }
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            code: ErrorCode::Internal,
            message: Self::INTERNAL_MESSAGE.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize, ts_rs::TS)]
pub enum ErrorCode {
    BadRequest = 4000,
    Invalid = 4001,
    Unauthorized = 4010,
    Forbidden = 4030,
    NotFound = 4040,
    Conflict = 4090,
    #[default]
    Internal = 5000,
    ServiceUnavailable = 5030,
    GatewayTimeout = 5040,
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Protocol(_) => ErrorResponse {
                code: ErrorCode::BadRequest,
                message: "Protocol error - Bad Request".to_string(),
            },
            sqlx::Error::ColumnIndexOutOfBounds { .. } => ErrorResponse {
                code: ErrorCode::BadRequest,
                message: "Column index out of bounds - Bad Request".to_string(),
            },
            sqlx::Error::ColumnNotFound(_) => ErrorResponse {
                code: ErrorCode::BadRequest,
                message: "Column not found - Bad Request".to_string(),
            },
            sqlx::Error::ColumnDecode { .. } => ErrorResponse {
                code: ErrorCode::BadRequest,
                message: "Column decode error - Bad Request".to_string(),
            },
            sqlx::Error::Decode(_) => ErrorResponse {
                code: ErrorCode::BadRequest,
                message: "Decode error - Bad Request".to_string(),
            },
            sqlx::Error::RowNotFound => ErrorResponse {
                code: ErrorCode::NotFound,
                message: "Entry not found".to_string(),
            },
            sqlx::Error::PoolClosed => ErrorResponse {
                code: ErrorCode::ServiceUnavailable,
                message: "Database pool closed - Service Unavailable".to_string(),
            },
            sqlx::Error::Io(_) => ErrorResponse {
                code: ErrorCode::ServiceUnavailable,
                message: "Database I/O error - Service Unavailable".to_string(),
            },
            sqlx::Error::Tls(_) => ErrorResponse {
                code: ErrorCode::ServiceUnavailable,
                message: "Database TLS error - Service Unavailable".to_string(),
            },
            sqlx::Error::PoolTimedOut => ErrorResponse {
                code: ErrorCode::GatewayTimeout,
                message: "Database pool timed out (504 Gateway Timeout".to_string(),
            },
            _ => ErrorResponse::default(),
        }
    }
}

impl From<ValidationError> for ErrorResponse {
    fn from(value: ValidationError) -> Self {
        Self {
            code: ErrorCode::Invalid,
            message: value
                .message
                .map(|v| v.to_string())
                .unwrap_or_else(|| format!("The fields you gave are not valid ({})", value.code)),
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

        let message = if messages.is_empty() {
            "The fields you gave are not valid".to_string()
        } else {
            messages.join("\n")
        };

        Self {
            code: ErrorCode::Invalid,
            message,
        }
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
impl_from_report_for_error_response!(sqlx::Error, ValidationError, ValidationErrors);
