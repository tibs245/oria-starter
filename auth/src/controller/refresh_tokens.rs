use std::sync::Arc;
use axum::{Extension, Json};
use crate::entities::error::AuthError;
use crate::services::{AuthTokensService};
use crate::views::payload::RefreshTokenPayload;
use crate::views::response::AuthBody;

pub async fn refresh_tokens<AuthServiceImpl: AuthTokensService>(auth_service: Extension<Arc<AuthServiceImpl>>, Json(payload): Json<RefreshTokenPayload>) -> Result<Json<AuthBody>, AuthError> {
    let tokens = auth_service.refresh_tokens(payload).await?;
    Ok(Json(tokens))
}