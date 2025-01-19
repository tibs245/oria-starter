use crate::services::{UserGetService};
use auth_module::views::error_response::handle_error;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::{Extension, Json};
use std::sync::Arc;
use auth_module::entities::AuthSession;
use crate::views::response::UserPrivateDetails;

pub async fn get_own_profile<UserServiceImpl: UserGetService>(
    Extension(user_service): Extension<Arc<UserServiceImpl>>,
    Extension(auth_session): Extension<AuthSession>,
) -> Result<(StatusCode, Json<UserPrivateDetails>), ErrorResponse> {
    println!("Session : {}\n", auth_session.username);
    let user_created = user_service
        .get_user(&auth_session.username)
        .await
        .map_err(|error| handle_error(error))?;

    Ok((StatusCode::CREATED, Json(user_created.into())))
}
