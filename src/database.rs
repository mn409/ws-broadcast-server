use sqlx::PgPool;

pub async fn connect_db(db_url: &str) -> PgPool {
    PgPool::connect(db_url)
        .await
        .expect("Failed to connect to DB. Check DATABASE_URL and that Postgres is running.")
}