#[derive(Debug, Clone)]
pub struct ApiKey(pub String);

impl Into<String> for ApiKey {
    fn into(self) -> String {
        self.0
    }
}
