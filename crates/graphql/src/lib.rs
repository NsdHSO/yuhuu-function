mod schema;
mod query;
mod mutation;
mod strapi_client;
mod types;
mod handlers;

pub use schema::build_schema;
pub use types::*;
pub use strapi_client::StrapiClient;
pub use handlers::{graphql_handler, graphql_playground, strapi_proxy_handler};

pub type GraphQLSchema = async_graphql::Schema<query::QueryRoot, mutation::MutationRoot, async_graphql::EmptySubscription>;
