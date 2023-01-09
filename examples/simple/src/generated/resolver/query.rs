use crate::generated::schema::query::Query;
use crate::generated::schema::viewer::Viewer;

pub async fn query_viewer_resolver(_: &Query) -> Viewer {
    Viewer {
        name: "John Doe".to_string(),
    }
}
