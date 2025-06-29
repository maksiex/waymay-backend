use axum::{
    routing::{post, patch},
    Router,
};
use crate::auth::handlers::{reg, login};
use sqlx::PgPool;

pub fn router(state: PgPool) -> Router {
    Router::new()
        .route("/reg", post(reg))
        .route("/login", patch(login))
        .with_state(state)
}
