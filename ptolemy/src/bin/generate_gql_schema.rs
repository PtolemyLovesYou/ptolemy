use ptolemy::api::{Mutation, Query};
use juniper::{EmptySubscription, RootNode};

const DEFAULT_DIR: &str = "graphql/schema.gql";

fn main() {
    let schema = RootNode::new(Query {}, Mutation {}, EmptySubscription::<()>::new());

    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or(DEFAULT_DIR.to_string());
    println!("PRINTING TO {}", output_dir);

    std::fs::write(output_dir, schema.as_sdl()).expect("Failed to write schema");
}
