use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
}

impl State {
    pub async fn new(database_url: String) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPool::connect(&database_url).await?,
        })
    }
}
