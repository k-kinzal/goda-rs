use async_graphql::{EmptyMutation, EmptySubscription, SimpleObject};

#[derive(SimpleObject)]
pub struct Query {
    hi: String,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            hi: "world".to_string(),
        }
    }
}

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;
