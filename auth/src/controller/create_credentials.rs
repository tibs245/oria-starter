use std::sync::Arc;
use crate::views::error_response::handle_error;
use crate::views::payload::LoginPayload;
use crate::views::response::CredentialsPrivateDetails;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::Json;
use crate::services::AuthCreateCredentialsService;

pub async fn create_credentials<
    AuthServiceImpl: AuthCreateCredentialsService,
>(
    Extension(auth_service): Extension<Arc<AuthServiceImpl>>,
    Json(login_payload): Json<LoginPayload>,
) -> Result<(StatusCode, Json<CredentialsPrivateDetails>), ErrorResponse> {
    let user_created = auth_service.create_credentials(login_payload)
        .await
        .map_err(|error| handle_error(error))?;

    Ok((StatusCode::CREATED, Json(user_created.into())))
}
