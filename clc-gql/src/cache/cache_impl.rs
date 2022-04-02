use clc_engine::{Calculator, Error};
use redis::{aio::MultiplexedConnection, Client};

use crate::errors::CacheError;

pub struct Cache {
    calculator: Calculator,
    con: MultiplexedConnection,
}

impl Cache {
    pub(super) async fn connect(calculator: Calculator, addr: &str) -> Result<Self, CacheError> {
        tracing::info!(addr, "Connecting to redis...");

        let client = Client::open(addr)?;
        let con = client.get_multiplexed_tokio_connection().await?;

        tracing::info!(addr, "Successfully connected to redis");
        Ok(Cache::new(calculator, con))
    }

    pub(super) fn new(calculator: Calculator, con: MultiplexedConnection) -> Self {
        Self { calculator, con }
    }

    pub(super) async fn get_eval(&self, expression: &str) -> Result<f64, Error> {
        let mut con = self.con.clone();

        // Option<T> impl FromRedisValue and handle nil from redis.
        let cache: Result<Option<f64>, _> = redis::cmd("GET")
            .arg(expression)
            .query_async(&mut con)
            .await
            .map_err(CacheError::from);

        match cache {
            Ok(Some(cache)) => {
                tracing::info!(%expression, "Cache hit");
                return Ok(cache);
            }
            Ok(None) => (),
            Err(err) => {
                tracing::warn!(?err);
            }
        }

        let eval = self.calculator.calculate_line(expression)?;

        if let Err(err) = redis::cmd("SETEX")
            .arg(expression)
            .arg(3600)
            .arg(eval)
            .query_async::<_, ()>(&mut con)
            .await
        {
            tracing::warn!(?err);
        }

        Ok(eval)
    }
}
