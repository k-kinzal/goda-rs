use crate::generated::types::Response;
use sha2::{Digest, Sha256};

mod operation8ad39c6d58c4a84c7c09095aea5c4977fd39e7e2f929ca869d7da93e0d1d834c;

pub async fn resolve(request: crate::generated::types::Request) -> Response {
    let operation_id = format!("{:x}", Sha256::digest(request.body().query.as_bytes()));

    println!("operation_id: {}", operation_id);
    let v = match operation_id.as_str() {
        "operation8ad39c6d58c4a84c7c09095aea5c4977fd39e7e2f929ca869d7da93e0d1d834c" => {
            let response = operation8ad39c6d58c4a84c7c09095aea5c4977fd39e7e2f929ca869d7da93e0d1d834c::Operation::default().resolve().await;
            serde_json::to_value(response).unwrap()
        }
        _ => todo!(),
    };

    Response::new(v)
}
