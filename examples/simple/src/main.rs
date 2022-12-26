use std::net::SocketAddr;

use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription};
use http::{Method, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::{body::Bytes, service::service_fn, Request, Response};
use log::trace;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;

use goda::Registry;

use crate::operations::OperationA;
use crate::schema::{Query, Schema};

mod operations;
mod schema;

type OperationRegistry = Registry<async_graphql::Request, async_graphql::Response>;

async fn action(
    schema: &Schema,
    registry: &OperationRegistry,
    request: async_graphql::Request,
) -> async_graphql::Response {
    let operation_id = if let Some(v) = request.extensions.get("persisted_query") {
        v.to_string()
    } else {
        format!("{:x}", Sha256::digest(request.query.as_bytes()))
    };
    let operation = registry.resolve(operation_id.as_str()).await;
    if let Ok(operation) = operation {
        trace!(
            "action: request and response are hooked now that the operation ID has been found: {}",
            operation_id
        );
        let request = operation.hook_request(request).await;
        let response = schema.execute(request).await;
        operation.hook_response(response).await
    } else {
        trace!("action: operation ID not found: {}", operation_id);
        schema.execute(request).await
    }
}

async fn handler(
    schema: &Schema,
    registry: &OperationRegistry,
    request: Request<Incoming>,
) -> Result<Response<Full<Bytes>>> {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/graphql") => {
            let mut builder = Request::builder();
            for (name, value) in request.headers() {
                builder = builder.header(name, value);
            }
            let request = builder.method(request.method()).uri(request.uri()).body(
                serde_json::from_slice::<async_graphql::Request>(
                    &request.collect().await?.to_bytes(),
                )?,
            )?;
            let (_, body) = request.into_parts();

            let resp = action(schema, registry, body).await;

            let mut builder = Response::builder().status(StatusCode::OK);
            for (name, value) in resp.http_headers.iter() {
                builder = builder.header(name, value);
            }
            Ok(builder.body(Full::new(Bytes::from(
                serde_json::to_string(&resp)?.into_bytes(),
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

    let registry = Registry::default().register::<OperationA>().await;

    let schema = Schema::build(
        Query::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .finish();

    loop {
        let (stream, _) = listener.accept().await?;
        let schema = schema.clone();
        let registry = registry.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    stream,
                    service_fn(|request| handler(&schema, &registry, request)),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
