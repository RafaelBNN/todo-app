use axum::{Router, routing::get, Json, Extension};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use dotenvy::dotenv;

#[derive(Serialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
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

async fn create_todo(
    Extension(pool): Extension<sqlx::PgPool>,
    Json(payload): Json<NewTodo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let rec = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, done)
        VALUES ($1, false)
        RETURNING id, title, done
        "#,
        payload.title
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        eprintln!("Erro ao criar tarefa: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar tarefa".to_string())
    })?;

    Ok((StatusCode::CREATED, Json(rec)))
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
        .route("/todos", get(list_todos).post(create_todo))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
