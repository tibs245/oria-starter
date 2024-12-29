use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UsernamePasswordPayload {
    pub username: String,
    pub password: String,
}
