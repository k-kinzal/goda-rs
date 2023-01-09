use crate::generated::schema::user::User;
use crate::generated::schema::viewer::Viewer;

pub async fn viewer_name_resolver(parent: &Viewer) -> String {
    parent.name.clone()
}

pub async fn viewer_friends_resolver(_: &Viewer) -> Vec<User> {
    vec![
        User {
            name: "John Doe1".to_string(),
        },
        User {
            name: "Jane Doe2".to_string(),
        },
        User {
            name: "Jane Doe3".to_string(),
        },
    ]
}
