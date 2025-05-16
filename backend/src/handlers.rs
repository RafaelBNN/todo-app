use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};

use crate::models::{NewTodo, Todo, UpdateTodo};

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

pub async fn update_todo(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::PgPool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let rec = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET done = $1
        WHERE id = $2
        RETURNING id, title, done
        "#,
        payload.done,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        eprintln!("Erro ao atualizar tarefa: {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao atualizar tarefa".to_string(),
        )
    })?;

    match rec {
        Some(todo) => Ok(Json(todo)),
        None => Err((StatusCode::NOT_FOUND, "Tarefa não encontrada".into())),
    }
}

pub async fn delete_todo(
    Path(id): Path<i32>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!(
        r#"
        DELETE FROM todos WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await
    .map_err(|err| {
        eprintln!("Erro ao deletar tarefa: {}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erro ao deletar tarefa".to_string(),
        )
    })?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Tarefa não encontrada".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
