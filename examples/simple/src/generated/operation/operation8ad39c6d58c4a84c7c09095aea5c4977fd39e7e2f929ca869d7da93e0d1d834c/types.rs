use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub viewer: Viewer,
}
#[derive(Serialize, Deserialize)]
pub struct Viewer {
    pub friends: Vec<User>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}
