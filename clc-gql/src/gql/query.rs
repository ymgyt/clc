use crate::cache::CacheLayer;
use crate::gql::errors::GraphqlError;
use async_graphql::{Context, Object, Result};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn eval<'ctx>(&self, ctx: &Context<'ctx>, expression: String) -> Result<f64> {
        let calc = ctx.data_unchecked::<CacheLayer>();
        calc.get_eval(&expression)
            .await
            .map_err(|err| GraphqlError::from(err).into())
    }
}
