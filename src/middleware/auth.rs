use tower_sessions_redis_store::{
    fred::prelude::*,
    RedisStore
};
use tower_sessions::{Expiry, SessionManagerLayer};
use std::env::var;
use anyhow::Result;
use tower_sessions::cookie::time::Duration;

pub async fn create_redis_pool() -> Result<Pool>{
    let redis_url = var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_config = Config::from_url(&redis_url)?;
    let pool =
        Pool::new(
            redis_config,
            None,
            None,
            None,
            10
        )?;
    pool.connect();
    pool.wait_for_connect().await?;
    Ok(pool)
}

pub fn auth_middleware(pool: Pool) -> Result<SessionManagerLayer<RedisStore<Pool>>>{
    let store = RedisStore::new(pool);
    let layer =
        SessionManagerLayer::new(store)
            .with_secure(false) // change to true in prod
            .with_expiry(
                Expiry::OnInactivity(Duration::minutes(10))
            );
    Ok(layer)
}