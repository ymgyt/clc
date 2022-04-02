use crate::cache::CacheLayer;
use crate::config;
use clc_engine::Calculator;

/// Dependency contains application dependencies.
pub struct Dependency {
    pub calculator: CacheLayer,
}

impl Dependency {
    pub async fn new() -> Self {
        let calc = Calculator::default();
        let con = std::env::var(config::env::REDIS_ENDPOINT).ok();
        let cache = CacheLayer::new(calc, con.as_deref()).await.unwrap();
        Self { calculator: cache }
    }
}
