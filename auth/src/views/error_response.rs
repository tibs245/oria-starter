use crate::datastore::{AuthDatastoreError, TokenDatastoreError};
use crate::entities::error::AuthError;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::error::Error;

trait HttpStatusCodeError {
    fn get_http_status_code(&self) -> StatusCode;
}

impl HttpStatusCodeError for AuthError {
    fn get_http_status_code(&self) -> StatusCode {
        match self {
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::WrongCredentials => StatusCode::BAD_GATEWAY,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => StatusCode::BAD_REQUEST,
            AuthError::ServerError => StatusCode::SERVICE_UNAVAILABLE,
            AuthError::Duplicated => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (self.get_http_status_code(), self.to_string()).into_response()
    }
}

/// Implement of IntoResponse for TokenDatastoreError
/// Used to format axum response
impl HttpStatusCodeError for AuthDatastoreError {
    fn get_http_status_code(&self) -> StatusCode {
        match self {
            AuthDatastoreError::BadFormat(_) => StatusCode::BAD_REQUEST,
            AuthDatastoreError::ProvidersError => StatusCode::UNAUTHORIZED,
            AuthDatastoreError::InternalError => StatusCode::BAD_GATEWAY,
        }
    }
}

/// Implement of IntoResponse for TokenDatastoreError
/// Used to format axum response
impl HttpStatusCodeError for TokenDatastoreError {
    fn get_http_status_code(&self) -> StatusCode {
        match self {
            TokenDatastoreError::ProvidersError => StatusCode::SERVICE_UNAVAILABLE,
            TokenDatastoreError::InternalError => StatusCode::BAD_GATEWAY,
        }
    }
}

// Function to handle errors
pub fn handle_error(err: Box<dyn Error + Sync + Send>) -> Response<Body> {
    let status_code = if let Some(auth_err) = err.downcast_ref::<AuthDatastoreError>() {
        auth_err.get_http_status_code()
    } else if let Some(token_err) = err.downcast_ref::<TokenDatastoreError>() {
        token_err.get_http_status_code()
    } else if let Some(token_err) = err.downcast_ref::<AuthError>() {
        token_err.get_http_status_code()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let body = Json(json!({
        "error": err.to_string(),
    }));
    // Default case
    (status_code, body).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_http_status_code_auth_wrong_credentials() {
        let err = AuthError::WrongCredentials;
        assert_eq!(err.get_http_status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_get_http_status_code_auth_missing_credentials() {
        let err = AuthError::MissingCredentials;
        assert_eq!(err.get_http_status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_get_http_status_code_auth_token_creation() {
        let err = AuthError::TokenCreation;
        assert_eq!(
            err.get_http_status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_get_http_status_code_auth_invalid_token() {
        let err = AuthError::InvalidToken;
        assert_eq!(err.get_http_status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_get_http_status_code_auth_server_error() {
        let err = AuthError::ServerError;
        assert_eq!(err.get_http_status_code(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_get_http_status_code_auth_datastore_bad_format() {
        let err = AuthDatastoreError::BadFormat(String::from("bad format"));
        assert_eq!(err.get_http_status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_get_http_status_code_auth_datastore_providers_error() {
        let err = AuthDatastoreError::ProvidersError;
        assert_eq!(err.get_http_status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_get_http_status_code_auth_datastore_internal_error() {
        let err = AuthDatastoreError::InternalError;
        assert_eq!(err.get_http_status_code(), StatusCode::BAD_GATEWAY);
    }

    #[test]
    fn test_get_http_status_code_token_datastore_providers_error() {
        let err = TokenDatastoreError::ProvidersError;
        assert_eq!(err.get_http_status_code(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn test_get_http_status_code_token_datastore_internal_error() {
        let err = TokenDatastoreError::InternalError;
        assert_eq!(err.get_http_status_code(), StatusCode::BAD_GATEWAY);
    }
}
