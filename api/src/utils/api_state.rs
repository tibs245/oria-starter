#[derive(Clone)]
pub struct ApiState {
    pub(crate) mongo_cluster: mongodb::Client,
}