use crate::generated::operation::operation8ad39c6d58c4a84c7c09095aea5c4977fd39e7e2f929ca869d7da93e0d1d834c::types::{Query, User, Viewer};
use crate::generated::schema::query::Query as QuerySchema;
use futures::executor::block_on;
use futures::FutureExt;

mod types;

// ```graphql
// query {
//   viewer {
//     friends {
//       name
//     }
//   }
// }
//
// ```
#[derive(Default)]
pub struct Operation {}
impl Operation {
    pub async fn resolve(&self) -> Query {
        Query {
            viewer: QuerySchema::default()
                .viewer()
                .then(|viewer| async move {
                    Viewer {
                        friends: viewer
                            .friends()
                            .then(|friends| async {
                                let v = friends
                                    .into_iter()
                                    .map(|friend| {
                                        let name = friend.name();
                                        let name = block_on(name);
                                        User { name }
                                    })
                                    .collect();
                                v
                            })
                            .await,
                    }
                })
                .await,
        }
    }
}
