mod utils;

use axum::Router;
use mongodb;
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use shuttle_runtime::SecretStore;
use auth_module::auth_router_builder::AuthRouterBuilder;
use auth_module::utils::settings::AuthSettings;
use user_module::user_router_builder::UserRouterBuilder;
use base64::Engine;
use base64::engine::general_purpose;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let mongodb_uri = secrets.get("MONGODB_CLUSTER_URI").expect("No MONGODB_CLUSTER_URI found in Secret.toml. See README");

    let paseto_secret_key = secrets.get("PASETO_SECRET_KEY").expect("No PASETO_SECRET_KEY found in Secret.toml. See README");
    let paseto_public_key = secrets.get("PASETO_PUBLIC_KEY").expect("No PASETO_PUBLIC_KEY found in Secret.toml. See README");


    AuthSettings::set_secret_key(&general_purpose::STANDARD.decode(paseto_secret_key).expect("Unable decode key to init AuthSettings"));
    AuthSettings::set_public_key(&general_purpose::STANDARD.decode(paseto_public_key).expect("Unable decode key to init AuthSettings"));

    let mut client_options =
        ClientOptions::parse(mongodb_uri).await.expect("Unable to parse MONGODB_CLUSTER_URI.");
    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    let mongodb_client_cluster = Client::with_options(client_options).expect("Unable to connect mongodb DATABASE.");


    let auth_router_module = AuthRouterBuilder::new(&mongodb_client_cluster.database(&secrets.get("MONGODB_AUTH_DATABASE").unwrap_or("auth".to_string())));
    let user_router_module = UserRouterBuilder::new(&mongodb_client_cluster.database(&secrets.get("MONGODB_AUTH_DATABASE").unwrap_or("auth".to_string())), &mongodb_client_cluster.database(&secrets.get("MONGODB_USER_DATABASE").unwrap_or("users".to_string())));


    let app: Router<()> = Router::new()
        .nest("/auth", auth_router_module.into_router())
        .nest("/user", user_router_module.into_router());

    Ok(app.into())
}
