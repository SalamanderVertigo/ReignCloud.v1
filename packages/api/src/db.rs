#[cfg(not(target_arch = "wasm32"))]
use sqlx::PgPool;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::OnceCell;

#[cfg(not(target_arch = "wasm32"))]
static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Get a reference to the database pool.
/// Lazily initializes the connection and runs migrations on first call.
#[cfg(not(target_arch = "wasm32"))]
pub async fn pool() -> &'static PgPool {
    DB_POOL
        .get_or_init(|| async {
            dotenvy::dotenv().ok();
            let database_url =
                std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

            let pool = PgPool::connect(&database_url)
                .await
                .expect("Failed to connect to database");

            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("Failed to run migrations");

            println!("Database connected and migrations applied");
            pool
        })
        .await
}
