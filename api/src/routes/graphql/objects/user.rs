use crate::models::auth::models::User;
use juniper::graphql_object;

#[graphql_object]
impl User {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn username(&self) -> String {
        self.username.clone()
    }

    fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    fn status(&self) -> String {
        format!("{:?}", self.status)
    }

    fn is_admin(&self) -> bool {
        self.is_admin
    }

    fn is_sysadmin(&self) -> bool {
        self.is_sysadmin
    }
}
