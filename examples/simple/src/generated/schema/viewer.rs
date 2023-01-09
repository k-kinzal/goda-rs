use crate::generated::resolver::viewer::{viewer_friends_resolver, viewer_name_resolver};
use crate::generated::schema::user::User;

#[derive(Clone)]
pub struct Viewer {
    pub name: String,
}

impl Viewer {
    pub async fn name(&self) -> String {
        viewer_name_resolver(self).await
    }

    pub async fn friends(&self) -> Vec<User> {
        viewer_friends_resolver(self).await
    }
}
