use axum::{routing::get, Router};
use crate::place::handlers::get_city_info;
use sqlx::PgPool;
pub fn router(state: PgPool) -> Router {
    Router::new()
        .route("/place", get(get_city_info))
        .with_state(state)
}
