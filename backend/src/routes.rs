use axum::routing::{get, put};
use axum::{Extension, Router};
use sqlx::PgPool;

use crate::handlers::{create_todo, list_todos, update_todo, delete_todo};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", put(update_todo).delete(delete_todo))
        .layer(Extension(pool))
}
