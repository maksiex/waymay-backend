use axum:: {
    routing::{get, post, patch},
    Router,
};
use sqlx::{postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};


mod auth;
mod constants;
mod models;
mod place;

use constants::errors::{ENV_LOAD_ERROR, DB_CONNECTION_ERROR};
use crate::auth::handlers::{reg, login};
use crate::place::handlers::get_city_info;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    dotenvy::dotenv().expect(ENV_LOAD_ERROR);
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect(DB_CONNECTION_ERROR);
    
    let listener = TcpListener::bind(server_address)
    .await
        .expect("Can't create TCP listener");
    println!("Listening on {}", listener.local_addr().unwrap());
    
    let app = Router::new()
        .route("/", get(|| async {"Hello from router"}))
        .route("/reg", post(reg))
        .route("/login", patch(login))
        .route("/place", get(get_city_info))
        .layer(cors)
        .with_state(db_pool);
    
    axum::serve(listener, app) 
    .await
    .expect("Error serving application");
}
