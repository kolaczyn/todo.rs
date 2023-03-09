use std::env;

use anyhow::Error;
use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
}

impl State {
    pub async fn new() -> Result<Self, Error> {
        let database_url = env::var("DATABASE_URL")?;
        Ok(Self {
            pool: PgPool::connect(&database_url).await?,
        })
    }
}
