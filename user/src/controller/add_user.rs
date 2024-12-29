use crate::services::{UserAddService};
use crate::views::user_payload::UserWithCredentialsPayload;
use auth_module::views::error_response::handle_error;
use auth_module::views::response::UserPrivateDetails;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn add_user<UserServiceImpl: UserAddService>(
    Extension(user_service): Extension<Arc<UserServiceImpl>>,
    Json(user_with_credential_payload): Json<UserWithCredentialsPayload>,
) -> Result<(StatusCode, Json<UserPrivateDetails>), ErrorResponse> {
    let user_created = user_service
        .add_user(user_with_credential_payload)
        .await
        .map_err(|error| handle_error(error))?;

    Ok((StatusCode::CREATED, Json(user_created.into())))
}
