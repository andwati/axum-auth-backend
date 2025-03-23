use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(PartialEq, Debug)]
pub enum ErrorMesaage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNolongerExists,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl ToString for ErrorMesaage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMesaage {
    fn to_str(&self) -> String {
        match self {
            ErrorMesaage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMesaage::ExceededMaxPasswordLength(length) => {
                { format!("Password length exceeded. Max length is {}", length) }.to_string()
            }
            ErrorMesaage::HashError => "Error hashing password".to_string(),
            ErrorMesaage::InvalidToken => "Invalid token".to_string(),
            ErrorMesaage::ServerError => "Internal server error".to_string(),
            ErrorMesaage::WrongCredentials => "Wrong credentials".to_string(),
            ErrorMesaage::EmailExists => "Email already exists".to_string(),
            ErrorMesaage::UserNolongerExists => "User no longer exists".to_string(),
            ErrorMesaage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMesaage::PermissionDenied => "Permission denied".to_string(),
            ErrorMesaage::UserNotAuthenticated => "User not authenticated".to_string(),
            ErrorMesaage::InvalidHashFormat => "Invalid hash format".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn serve_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}
