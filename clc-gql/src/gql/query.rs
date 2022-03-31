use crate::gql::errors::GraphqlError;
use async_graphql::{Context, Object, Result};
use clc_engine::Calculator;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn eval<'ctx>(&self, ctx: &Context<'ctx>, expression: String) -> Result<f64> {
        let calc = ctx.data_unchecked::<Calculator>();
        calc.calculate_line(&expression)
            .map_err(|err| GraphqlError::from(err).into())
    }
}
