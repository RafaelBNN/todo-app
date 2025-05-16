use axum::{routing::get, Extension, Router};
use sqlx::PgPool;

use crate::handlers::{create_todo, list_todos};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .layer(Extension(pool))
}
