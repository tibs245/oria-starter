use crate::controller::create_credentials::create_credentials;
use crate::controller::login::login;
use crate::controller::refresh_tokens::refresh_tokens;
use crate::datastore::mongo::tokens::MongoTokenDatastore;
use crate::datastore::mongo::users::MongoAuthDatastore;
use crate::datastore::{AuthDatastore, TokenDatastore};
use crate::services::AuthService;
use axum::routing::post;
use axum::{Extension, Router};
use mongodb::Database;
use std::sync::Arc;
use crate::entities::Privileges;
use crate::layer::claims::AuthGuardLayer;

trait AuthServiceProvider<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore> {
    fn get_auth_service(&self) -> Arc<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>>;
}

pub enum AuthActions {
    Subscribe,
    CreateRole,
    Get(String),
}

pub struct AuthRouterBuilder<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore> {
    auth_service: Arc<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>>,
}

impl AuthRouterBuilder<MongoAuthDatastore, MongoTokenDatastore> {
    pub fn new(mongo_db: &Database) -> Self {
        let auth_datastore = MongoAuthDatastore::new(mongo_db);
        let token_datastore = MongoTokenDatastore::new(mongo_db);
        Self {
            auth_service: Arc::new(AuthService::new(auth_datastore, token_datastore)),
        }
    }
}

impl<AuthDatastoreImpl: AuthDatastore, TokenDatastoreImpl: TokenDatastore> AuthServiceProvider<AuthDatastoreImpl, TokenDatastoreImpl> for AuthRouterBuilder<AuthDatastoreImpl, TokenDatastoreImpl> {
    fn get_auth_service(&self) -> Arc<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>> {
        self.auth_service.clone()
    }
}

impl<AuthDatastoreImpl, TokenDatastoreImpl> AuthRouterBuilder<AuthDatastoreImpl, TokenDatastoreImpl>
where
    AuthDatastoreImpl: AuthDatastore + 'static + Clone + Send + Sync,
    TokenDatastoreImpl: TokenDatastore + 'static + Clone + Send + Sync,
{
    pub fn into_router(self) -> Router {
        Router::new()
            .route(
                "/create_credentials",
                post(create_credentials::<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>>).layer(AuthGuardLayer { privileges: Privileges::Anonymous }),
            )
            .route(
                "/login",
                post(login::<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>>).layer(AuthGuardLayer { privileges: Privileges::Anonymous }),
            )
            .route(
                "/refresh_token",
                post(refresh_tokens::<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>>).layer(AuthGuardLayer { privileges: Privileges::Allow }),
            )
            .layer(Extension(self.auth_service))
    }
}
