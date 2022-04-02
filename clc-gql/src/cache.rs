#[cfg(feature = "cache")]
mod cache_impl;

use clc_engine::{Calculator, Error};

use crate::errors::CacheError;

pub enum CacheLayer {
    Nocache(Calculator),
    #[cfg(feature = "cache")]
    Cache(cache_impl::Cache),
}

impl CacheLayer {
    pub async fn new(calculator: Calculator, cache_addr: Option<&str>) -> Result<Self, CacheError> {
        #[cfg(not(feature = "cache"))]
        {
            let _ = cache_addr; // For unused variable lint
            return Ok(CacheLayer::Nocache(calculator));
        }

        #[cfg(feature = "cache")]
        {
            match cache_addr {
                Some(addr) => cache_impl::Cache::connect(calculator, addr)
                    .await
                    .map(CacheLayer::Cache),
                None => {
                    tracing::info!("Redis url not found, skip connection to redis");

                    Ok(CacheLayer::Nocache(calculator))
                }
            }
        }
    }

    pub async fn get_eval(&self, expression: &str) -> Result<f64, Error> {
        match self {
            CacheLayer::Nocache(cal) => cal.calculate_line(expression),
            #[cfg(feature = "cache")]
            CacheLayer::Cache(cache) => cache.get_eval(expression).await,
        }
    }
}

#[cfg(test)]
impl CacheLayer {
    pub fn no_cache(calc: Calculator) -> Self {
        CacheLayer::Nocache(calc)
    }
}
