use axum::http::StatusCode;
use axum::{Extension, Json};

use crate::models::{NewTodo, Todo};

pub async fn list_todos(Extension(pool): Extension<sqlx::PgPool>) -> Json<Vec<Todo>> {
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

pub async fn create_todo(
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao criar tarefa".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(rec)))
}
