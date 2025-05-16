use axum::{Router, routing::get, Json, Extension};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use dotenvy::dotenv;

#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

async fn list_todos(Extension(pool): Extension<sqlx::PgPool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, done FROM todos ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_else(|err| {
        eprintln!("Erro ao buscar tarefas: {}", err);
        vec![]
    });

    Json(todos)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida no .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/todos", get(list_todos))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
