use sqlx::PgPool;
use crate::todo::{Todo, CreateTodo, UpdateTodo};

type Result<T> = core::result::Result<T, sqlx::Error>;

pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todo")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<Todo>> {
    sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn insert(todo: &CreateTodo, pool: &PgPool) -> Result<Todo> {
    let CreateTodo {
        title,
        description,
    } = todo;

    sqlx::query_as::<_, Todo>(
        r#"
            INSERT INTO todo (title, description)
            VALUES ($1, $2)
            RETURNING *
        "#
    )
        .bind(title)
        .bind(description)
        .fetch_one(pool)
        .await
}

pub async fn update_by_id(id: i32, todo: &UpdateTodo, pool: &PgPool) -> Result<Option<Todo>> {
    let UpdateTodo {
        title,
        description,
        completed,
    } = todo;

    sqlx::query_as::<_, Todo>(
        r#"
            UPDATE todo SET
            title = $1,
            description = $2,
            completed = $3
            WHERE id = $4
            RETURNING *
        "#
    )
        .bind(title)
        .bind(description)
        .bind(completed)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_by_id(id: i32, pool: &PgPool) -> Result<Option<Todo>> {
    sqlx::query_as::<_, Todo>(
        r#"
            DELETE FROM todo
            WHERE id = $1
            RETURNING *
        "#
    )
        .bind(id)
        .fetch_optional(pool)
        .await
}