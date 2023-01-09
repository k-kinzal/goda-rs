use crate::generated::resolver::user::user_name_resolver;

#[derive(Clone)]
pub struct User {
    pub name: String,
}
impl User {
    pub async fn name(&self) -> String {
        user_name_resolver(self).await
    }
}
