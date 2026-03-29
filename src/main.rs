use mini_s3::http::{AppState, router};
use mini_s3::infrastructure::database;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mini_s3=debug".parse()?))
        .init();
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:database.db".to_string());
    let pool = database::connect(&database_url).await?;

    database::run_migrations(&pool).await?;

    let state = AppState::new(pool);
    let app = router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
