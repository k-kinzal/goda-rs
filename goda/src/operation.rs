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
/// impl Operation<Request, Response> for OperationA {
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
///     async fn hook_request(&self, mut request: async_graphql::Request) -> async_graphql::Request {
///         request
///     }
///   
///     async fn hook_response(
///         &self,
///         mut response: async_graphql::Response,
///     ) -> async_graphql::Response {
///         response
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Operation<Request, Response>: OperationClone<Request, Response> + Send + Sync {
    fn query(&self) -> String;
    fn operation_id(&self) -> String;
    async fn hook_request(&self, request: Request) -> Request;
    async fn hook_response(&self, response: Response) -> Response;
}

pub trait OperationClone<Request, Response> {
    fn clone_box(&self) -> Box<dyn Operation<Request, Response>>;
}

impl<T, Request, Response> OperationClone<Request, Response> for T
where
    T: 'static + Operation<Request, Response> + Clone,
{
    fn clone_box(&self) -> Box<dyn Operation<Request, Response>> {
        Box::new(self.clone())
    }
}
