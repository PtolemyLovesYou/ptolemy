const DEFAULT_DIR: &str = "graphql/schema.gql";

fn main() {
    let schema = api::graphql_schema!().finish();

    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or(DEFAULT_DIR.to_string());
    println!("PRINTING TO {}", output_dir);

    std::fs::write(output_dir, schema.sdl()).expect("Failed to write schema");
}
