use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::{get, post};
use mongodb::Database;
use auth_module::datastore::{AuthDatastore, TokenDatastore};
use auth_module::datastore::mongo::tokens::MongoTokenDatastore;
use auth_module::datastore::mongo::users::MongoAuthDatastore;
use auth_module::entities::Privileges;
use auth_module::layer::claims::AuthGuardLayer;
use auth_module::services::{AuthCreateCredentialsService, AuthGetCredentialsService, AuthService, AuthTokensService, AuthValidCredentialsService};
use crate::controller::add_user::add_user;
use crate::controller::get_own_profile::get_own_profile;
use crate::datastore::mongo::MongoUserDatastore;
use crate::datastore::UserDatastore;
use crate::services::{UserService};

pub enum UsersActions {
    Create,
    Get(String),
}
pub struct UserRouterBuilder<AuthServiceImpl: AuthCreateCredentialsService + AuthGetCredentialsService, UserDatastoreImpl: UserDatastore> {
    user_service: Arc<UserService<AuthServiceImpl, UserDatastoreImpl>>,
    rules: HashMap<UsersActions, fn() -> bool>,
}

impl UserRouterBuilder<AuthService<MongoAuthDatastore, MongoTokenDatastore>, MongoUserDatastore> {
    pub fn new(auth_mongo_db: &Database, user_mongo_db: &Database) -> Self {
        let user_datastore = MongoUserDatastore::new(user_mongo_db);
        let auth_service = Self::build_auth_service(auth_mongo_db);

        Self {
            user_service: Arc::new(UserService::new(auth_service, user_datastore)),
            rules: Default::default(),
        }
    }

    fn build_auth_service(mongo_db: &Database) -> AuthService<MongoAuthDatastore, MongoTokenDatastore> {
        let auth_datastore = MongoAuthDatastore::new(mongo_db);
        let token_datastore = MongoTokenDatastore::new(mongo_db);

        AuthService::new(
            auth_datastore,
            token_datastore,
        )
    }
}


impl<UserDatastoreImpl, AuthDatastoreImpl, TokenDatastoreImpl>
UserRouterBuilder<AuthService<AuthDatastoreImpl, TokenDatastoreImpl>, UserDatastoreImpl>
where
    UserDatastoreImpl: UserDatastore + Send + Sync + Clone + 'static,
    AuthDatastoreImpl: AuthDatastore + Send + Sync + Clone + 'static,
    TokenDatastoreImpl: TokenDatastore + Send + Sync + Clone + 'static,
{
    pub fn into_router(self) -> Router {
        Router::new()
            .route(
                "/subscribe",
                post(add_user::<UserService<AuthService<MongoAuthDatastore, MongoTokenDatastore>, UserDatastoreImpl>>).layer(AuthGuardLayer { privileges: Privileges::Anonymous }),
            )
            .route(
                "/me",
                get(get_own_profile::<UserService<AuthService<MongoAuthDatastore, MongoTokenDatastore>, UserDatastoreImpl>>).layer(AuthGuardLayer { privileges: Privileges::Authenticated }),
            )
            .layer(Extension(self.user_service))
    }
}