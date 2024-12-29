use std::task::{Context, Poll};
use axum::{async_trait, RequestPartsExt};
use axum::body::{to_bytes, Body};
use axum::extract::FromRequestParts;
use axum::http::{Request};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use futures_util::future::BoxFuture;
use tower::{Layer, Service};
use crate::entities::error::AuthError;
use crate::entities::{AuthSession, Privileges, Roles};
use crate::utils::auth_claims::{AuthClaims};
use crate::utils::validate_token::{IntoClaims, TokenString};

const ANONYMOUS_USERNAME: &str = "anonymous";
#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        let untrusted_token = TokenString(bearer.token().to_string());
        let claims = untrusted_token.try_into_claims()?;

        AuthClaims::try_from(&claims).map_err(|_| AuthError::InvalidToken)
    }
}

#[derive(Clone)]
pub struct AuthGuardLayer {
    pub privileges: Privileges,
}

impl<S> Layer<S> for AuthGuardLayer {
    type Service = AuthGuardService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthGuardService { inner, privileges_required: self.privileges.clone() }
    }
}

#[derive(Clone)]
pub struct AuthGuardService<S> {
    inner: S,
    privileges_required: Privileges,
}
impl<S, B> Service<Request<B>> for AuthGuardService<S>
where
    S: Service<Request<B>, Response=Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        if self.privileges_required == Privileges::Deny {
            return Box::pin(async move {
                Ok(AuthError::Unauthorized.into_response())
            });
        }

        let (mut parts, body) = request.into_parts();
        let privileges_required = self.privileges_required.clone();
        let mut svc = self.inner.clone();

        Box::pin(async move {
            let auth_claims = AuthClaims::from_request_parts(&mut parts, &()).await;

            match auth_claims {
                Ok(auth_claims) => {
                    if auth_claims.role.is_none() {
                        return Ok(AuthError::InvalidToken.into_response());
                    }

                    let user_role = auth_claims.role.unwrap_or_else(|| Roles::None);

                    if !user_role.clone().is_authorized(privileges_required) {
                        return Ok(AuthError::Unauthorized.into_response());
                    }
                    parts.extensions.insert(AuthSession { username: auth_claims.username, role: user_role });
                }
                Err(error) => {
                    if privileges_required != Privileges::Anonymous && error != AuthError::MissingCredentials {
                        return Ok(AuthError::Unauthorized.into_response());
                    }
                    parts.extensions.insert(AuthSession { username: ANONYMOUS_USERNAME.to_string(), role: Roles::None });
                }
            }

            let request = Request::from_parts(parts.clone(), body);

            svc.call(request).await
        })
    }
}