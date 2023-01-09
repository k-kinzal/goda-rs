use std::net::SocketAddr;

use anyhow::Result;
use http::{Method, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{body::Bytes, service::service_fn, Request, Response};
use tokio::net::TcpListener;

mod generated;
mod operations;
mod schema;

async fn action(request: crate::generated::types::Request) -> crate::generated::types::Response {
    // let operation_id = if let Some(v) = request.body().extensions.get("persisted_query") {
    //     v.to_string()
    // } else {
    //     format!("{:x}", Sha256::digest(request.body().query.as_bytes()))
    // };
    // let operation = registry.resolve(operation_id.as_str());
    // if let Some(operation) = operation {
    // trace!(
    //     "action: request and response are hooked now that the operation ID has been found: {}",
    //     operation_id
    // );
    // let request = operation.hook_request(request).await;
    // let response = schema.execute(request).await;
    // operation.hook_response(response).await
    // let request = operation.hook_request(request).await;
    //     let response = operation.resolve().await;
    //     let value = async_graphql::Value::from_json(response).unwrap();
    //     async_graphql::Response::new(value)
    // } else {
    //     // println!("action: operation ID not found: {}", operation_id);
    //     // schema.execute(request).await
    //     todo!()
    // }
    crate::generated::operation::resolve(request).await
}

async fn handler(request: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/graphql") => {
            let mut builder = Request::builder();
            for (name, value) in request.headers() {
                builder = builder.header(name, value);
            }
            let request = builder.method(request.method()).uri(request.uri()).body(
                serde_json::from_slice::<crate::generated::types::Body>(
                    &request.collect().await?.to_bytes(),
                )?,
            )?;

            let resp = action(request).await;

            let builder = Response::builder().status(StatusCode::OK);
            // for (name, value) in resp.http_headers.iter() {
            //     builder = builder.header(name, value);
            // }
            Ok(builder.body(Full::new(Bytes::from(
                serde_json::to_string(&resp.body())?.into_bytes(),
            )))?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from(Bytes::new()))?),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    // let schema = Schema::build(
    //     Query::default(),
    //     EmptyMutation::default(),
    //     EmptySubscription::default(),
    // )
    // .finish();

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(|request| handler(request)))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
