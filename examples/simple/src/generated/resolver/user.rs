use crate::generated::schema::user::User;

pub async fn user_name_resolver(parent: &User) -> String {
    parent.name.clone()
}
