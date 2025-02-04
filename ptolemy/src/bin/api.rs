use ptolemy::api::{run_server, ServerError};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    run_server().await
}
