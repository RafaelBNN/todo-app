use axum::routing::{get, put};
use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{create_todo, list_todos, update_todo, delete_todo};

pub fn create_routes(pool: PgPool) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", put(update_todo).delete(delete_todo))
        .layer(Extension(pool))
        .layer(cors)
}
