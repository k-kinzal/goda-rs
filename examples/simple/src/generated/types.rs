use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub query: String,
    pub variables: serde_json::Value,
    // pub extensions: Option<HashMap<String, String>>,
}

pub type Request = http::Request<Body>;

pub type Response = http::Response<serde_json::Value>;
