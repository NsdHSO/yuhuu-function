use async_graphql::{EmptySubscription, Schema};
use crate::query::QueryRoot;
use crate::mutation::MutationRoot;
use crate::strapi_client::StrapiClient;

pub fn build_schema(strapi_client: StrapiClient) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(strapi_client)
        .finish()
}
