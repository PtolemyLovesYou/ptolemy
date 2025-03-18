use async_graphql::{Context, MergedObject, Object};

pub mod filter;
pub mod objects;
pub mod records;

#[derive(Debug, Default)]
pub struct BaseQuery;

#[Object]
impl BaseQuery {
    async fn ping<'ctx>(&self, _ctx: &Context<'ctx>) -> String {
        "Pong!".to_string()
    }
}

#[derive(Debug, Default, MergedObject)]
pub struct Query(pub BaseQuery, pub objects::IamQuery, pub records::Event);
