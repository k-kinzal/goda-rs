/// Operation is a trait that defines the interface for a GraphQL operation.
///
/// ```rust
/// # use crate::goda::Operation;
/// # use async_graphql::{Request, Response};
/// #
/// #[derive(Default, Copy, Clone)]
/// struct OperationA;
///
/// #[async_trait::async_trait]
/// impl Operation for OperationA {
///     fn query(&self) -> String {
///         r#"query HelloWorld {
///   hello
/// }
/// "#.to_string()
///     }
///   
///     fn operation_id(&self) -> String {
///         "ec438c4eba4f52bca3692c22884f329a9678baf10c7753b6f3d20071cfe62c93".to_string()
///     }
///
///     async fn resolve(self) -> serde_json::Value {
///         todo!()
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Operation: OperationClone + Send + Sync {
    fn query(&self) -> String;
    fn operation_id(&self) -> String;
    async fn resolve(&self) -> serde_json::Value;
}

pub trait OperationClone {
    fn clone_box(&self) -> Box<dyn Operation>;
}

impl<Target> OperationClone for Target
where
    Target: 'static + Operation + Clone,
{
    fn clone_box(&self) -> Box<dyn Operation> {
        Box::new(self.clone())
    }
}
