use api::graphql::Query;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

fn main() {
    let schema = RootNode::new(
        Query {},
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    std::fs::write("graphql/schema.gql", schema.as_sdl()).expect("Failed to write schema");
}
