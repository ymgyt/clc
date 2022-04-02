#[cfg(feature = "cache")]
use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[cfg(feature = "cache")]
    #[error("redis error")]
    Redis(#[from] RedisError),
}
