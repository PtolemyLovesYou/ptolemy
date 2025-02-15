use api::state::AppState;
use diesel_async::RunQueryDsl;

const QUERY: [&str; 6] = [
    "create role ptolemy_duckdb;",
    "grant connect on database ptolemy to ptolemy_duckdb;",
    "grant usage on schema public to ptolemy_duckdb;",
    "grant select on all tables in schema public to ptolemy_duckdb;",
    "grant usage on schema duckdb to ptolemy_duckdb;",
    "grant select on all tables in schema duckdb to ptolemy_duckdb;",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = match AppState::new().await {
        Ok(state) => state,
        Err(e) => panic!("Failed to create app state: {:?}", e),
    };

    let mut conn = state.get_conn().await.unwrap();

    for stmt in QUERY.into_iter() {
        println!("Executing: {}", stmt);
        if stmt.is_empty() {
            continue;
        }

        match diesel::sql_query(stmt).execute(&mut conn).await {
            Ok(_) => continue,
            Err(e) => println!("Failed to create ptolemy_duckdb role: {}", e),
        };
    }

    Ok(())
}
