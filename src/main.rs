mod config;
use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use config::Config;
use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Successfully connected to the database!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/health-status", get(health_check_handler))
        .with_state(Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
        }));

    println!("🚀 Server running on port 8000!");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Auth with Axum Rust is alive and well!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    let result = Json(json_response);

    result
}
