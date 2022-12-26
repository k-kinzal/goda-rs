use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::Result;

use crate::operation::Operation;

type BoxOperation<Request, Response> = Box<dyn Operation<Request, Response>>;

/// Registry of operations.
///
/// ```
/// # use async_graphql::{Request, Response};
/// # use goda::{Operation, Registry};
/// #
/// # #[derive(Default, Copy, Clone)]
/// # struct OperationA;
/// #
/// # #[async_trait::async_trait]
/// # impl Operation<Request, Response> for OperationA {
/// #     fn query(&self) -> String { unimplemented!()}
/// #     fn operation_id(&self) -> String { "ec438c4eba4f52bca3692c22884f329a9678baf10c7753b6f3d20071cfe62c93".to_string() }
/// #     async fn hook_request(&self, mut request: Request) -> Request { unimplemented!() }
/// #     async fn hook_response(&self, mut response: Response) -> Response { unimplemented!() }
/// # }
/// #
/// # #[tokio::main]
/// # async fn main() {
/// #   let request = Request::new("query HelloWorld { hello }");
///     let registry = Registry::default().register::<OperationA>().await;
///     let operation = registry.get("[persisted query hash]");
///     
///     let new_request = operation.hook_request(request).await;
/// # }
/// ```
pub struct Registry<Request, Response> {
    inner: Arc<RwLock<HashMap<String, BoxOperation<Request, Response>>>>,
}

impl<Request, Response> Default for Registry<Request, Response> {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<Request, Response> Clone for Registry<Request, Response> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Request, Response> Registry<Request, Response> {
    pub async fn register<O>(self) -> Self
    where
        O: Operation<Request, Response> + Default + 'static,
    {
        let v = O::default();
        self.inner
            .write()
            .unwrap_or_else(|_| {
                panic!(
                    "Registry: register::<{}>(): failed to acquire write lock",
                    std::any::type_name::<O>()
                )
            })
            .insert(v.operation_id(), Box::new(v));
        self
    }

    pub async fn resolve(
        &self,
        operation_id: &str,
    ) -> Result<Box<dyn Operation<Request, Response>>> {
        self.inner
            .read()
            .unwrap_or_else(|_| {
                panic!(
                    "Registry: resolve(operation_id: `{}`): failed to acquire read lock",
                    operation_id
                )
            })
            .get(operation_id)
            .map(|v| v.clone_box())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Registry: resolve(operation_id: `{}`): operation not found",
                    operation_id
                )
            })
    }
}
