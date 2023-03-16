use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;

use dotenv::dotenv;

async fn trunkate_tables(pool: &PgPool) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        TRUNCATE users, todos;
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn get_pool() -> Result<PgPool, anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

async fn insert_user(
    pool: &PgPool,
    id: i32,
    email: &str,
    password: &str,
) -> Result<(), anyhow::Error> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    sqlx::query!(
        r#"
        INSERT INTO users(id, email, password_hash)    
        VALUES($1, $2, $3)
        "#,
        id,
        email,
        hashed_password
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_users(pool: &PgPool) -> Result<(), anyhow::Error> {
    insert_user(pool, 101, "john@smith.com", "123456").await?;
    insert_user(pool, 102, "hannah_montana.net", "nikonikoniko").await?;

    Ok(())
}

async fn insert_todo(
    pool: &PgPool,
    id: i32,
    label: &str,
    completed: bool,
    user_id: i32,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO todos(id, label, completed, user_id)    
        VALUES($1, $2, $3, $4)
        "#,
        id,
        label,
        completed,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_todos(pool: &PgPool) -> Result<(), anyhow::Error> {
    insert_todo(pool, 101, "Learn Rust", false, 101).await?;
    insert_todo(pool, 102, "Learn React", true, 101).await?;
    insert_todo(pool, 103, "Learn Svelte", false, 101).await?;
    Ok(())
}

async fn seed(pool: &PgPool) -> Result<(), anyhow::Error> {
    trunkate_tables(pool).await?;
    insert_users(pool).await?;
    insert_todos(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv()?;

    let pool = get_pool().await?;
    seed(&pool).await?;

    Ok(())
}
