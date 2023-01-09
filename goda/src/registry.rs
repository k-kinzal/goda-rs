use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::Result;

use crate::operation::Operation;

type BoxOperationFactory = Box<fn() -> Box<dyn Operation>>;

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
/// # impl Operation for OperationA {
/// #     fn query(&self) -> String { unimplemented!()}
/// #     fn operation_id(&self) -> String { "ec438c4eba4f52bca3692c22884f329a9678baf10c7753b6f3d20071cfe62c93".to_string() }
/// #     async fn resolve(self) -> serde_json::Value { unimplemented!()}
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
pub struct Registry {
    inner: Arc<RwLock<HashMap<String, BoxOperationFactory>>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Clone for Registry {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Registry {
    pub async fn register<O>(self) -> Self
    where
        O: Operation + Default + 'static,
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
            .insert(v.operation_id(), Box::new(|| Box::new(O::default())));
        self
    }

    pub async fn resolve(&self, operation_id: &str) -> Result<Box<dyn Operation>> {
        self.inner
            .read()
            .unwrap_or_else(|_| {
                panic!(
                    "Registry: resolve(operation_id: `{}`): failed to acquire read lock",
                    operation_id
                )
            })
            .get(operation_id)
            .map(|factory| factory())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Registry: resolve(operation_id: `{}`): operation not found",
                    operation_id
                )
            })
    }
}
