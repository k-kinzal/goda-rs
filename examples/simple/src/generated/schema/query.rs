use crate::generated::resolver::query::query_viewer_resolver;
use crate::generated::schema::viewer::Viewer;

#[derive(Default, Clone, Copy)]
pub struct Query {}

impl Query {
    pub async fn viewer(&self) -> Viewer {
        query_viewer_resolver(self).await
    }
}
