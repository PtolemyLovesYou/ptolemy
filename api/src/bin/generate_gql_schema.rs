use api::graphql::{Query, Mutation};
use juniper::{EmptySubscription, RootNode};

fn main() {
    let schema = RootNode::new(
        Query {},
        Mutation {},
        EmptySubscription::<()>::new(),
    );

    std::fs::write("graphql/schema.gql", schema.as_sdl()).expect("Failed to write schema");
}
