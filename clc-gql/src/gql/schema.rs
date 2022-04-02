use async_graphql::{EmptyMutation, EmptySubscription, Schema, SchemaBuilder};

use crate::gql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CacheLayer;
    use async_graphql::{indexmap::indexmap, Name, Number, Value};
    use clc_engine::Calculator;

    macro_rules! f64 {
        ($value:expr) => {
            Value::Number(Number::from_f64($value).unwrap())
        };
    }

    #[tokio::test]
    async fn eval() {
        let schema = schema();
        assert_eq!(
            schema
                .execute(r#"query { eval(expression: "100 - 1") }"#)
                .await
                .data,
            Value::Object(indexmap! {
                Name::new("eval") => f64!(99.) ,
            }),
        );
    }

    fn schema() -> AppSchema {
        build()
            .data(CacheLayer::no_cache(Calculator::default()))
            .finish()
    }
}
