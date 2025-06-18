use axum::routing::{get, put};
use axum::{http, Extension, Router};
use http::header::{CONTENT_TYPE, ORIGIN};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{create_todo, delete_todo, list_todos, update_todo};

pub fn create_routes(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([CONTENT_TYPE, ORIGIN]);

    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", put(update_todo).delete(delete_todo))
        .layer(Extension(pool))
        .layer(cors)
}
