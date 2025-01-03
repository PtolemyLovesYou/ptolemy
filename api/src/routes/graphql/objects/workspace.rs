use crate::models::auth::models::Workspace;
use juniper::graphql_object;

#[graphql_object]
impl Workspace {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn archived(&self) -> bool {
        self.archived
    }

    fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}
