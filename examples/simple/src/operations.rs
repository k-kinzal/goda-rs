use serde_json::json;

use goda::Operation;

/// ```graphql
/// query HelloWorld {
///   hello
/// }
/// ```
#[derive(Default, Clone, Copy)]
pub struct OperationA;

#[async_trait::async_trait]
impl Operation<async_graphql::Request, async_graphql::Response> for OperationA {
    fn query(&self) -> String {
        r#"query HelloWorld {
  hello
}
"#
        .to_string()
    }

    fn operation_id(&self) -> String {
        "ec438c4eba4f52bca3692c22884f329a9678baf10c7753b6f3d20071cfe62c93".to_string()
    }

    async fn hook_request(&self, mut request: async_graphql::Request) -> async_graphql::Request {
        request.query = r#"
            query HelloWorld {
                hi
            }
        "#
        .to_string();
        request
    }

    async fn hook_response(
        &self,
        mut response: async_graphql::Response,
    ) -> async_graphql::Response {
        let json = response.data.into_json().unwrap();
        response.data = async_graphql::Value::from_json(json!({ "hello": json
            .as_object()
            .unwrap()
            .get("hi")
            .unwrap() }))
        .expect("unreachable");
        response
    }
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use async_graphql::{EmptyMutation, EmptySubscription};
    use serde_json::json;
    use sha2::{Digest, Sha256};

    use crate::schema::{Query, Schema};

    use super::*;

    #[test]
    fn test_operation_id() {
        let operation = OperationA::default();
        let expected = format!("{:x}", Sha256::digest(operation.query().as_bytes()));
        let actual = operation.operation_id();
        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_current_schema() -> anyhow::Result<()> {
        let schema = Schema::build(
            Query::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .finish();

        let operation = OperationA::default();
        let request = async_graphql::Request::new(operation.query());
        let request = operation.hook_request(request).await;
        let response = schema.execute(request).await;
        let response = operation.hook_response(response).await;
        let json = serde_json::to_string(&response)?;
        let expected = json!({
            "data": {
                "hello": "world"
            }
        });
        let actual = serde_json::from_str::<serde_json::Value>(&json)?;

        assert_json_eq!(expected, actual);

        Ok(())
    }
}
