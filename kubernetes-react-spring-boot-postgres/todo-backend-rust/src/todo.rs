use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, AppResult};
use std::ops::Deref;

pub fn configure_routes(config: &mut ServiceConfig) {
    use actix_web::web::*;

    config.service(
        resource("/todos")
            .route(get().to(find_all))
            .route(post().to(create))
    )
        .service(
            resource("/todos/{id}")
                .route(get().to(find_by_id))
                .route(put().to(update_by_id))
                .route(delete().to(delete_by_id))
        );
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: String,
    description: Option<String>,
    completed: bool,
}

async fn find_all(pool: Data<PgPool>) -> AppResult {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todo")
        .fetch_all(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(todos))
}

async fn find_by_id(path: Path<(i32, )>, pool: Data<PgPool>) -> AppResult {
    let (id, ) = path.into_inner();

    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        Err(AppError::NotFound(id))
    }
}

async fn create(todo: Json<CreateTodo>, pool: Data<PgPool>) -> AppResult {
    let CreateTodo {
        title,
        description,
    } = todo.deref();

    let todo = sqlx::query_as::<_, Todo>(
        r#"
            INSERT INTO todo (title, description)
            VALUES ($1, $2)
            RETURNING *
        "#
    )
        .bind(title)
        .bind(description)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(todo))
}

async fn update_by_id(path: Path<(i32, )>, todo: Json<UpdateTodo>, pool: Data<PgPool>) -> AppResult {
    let (id,) = path.into_inner();

    let UpdateTodo {
        title,
        description,
        completed,
    } = todo.deref();

    let todo = sqlx::query_as::<_, Todo>(
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
        .fetch_optional(pool.get_ref())
        .await?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        Err(AppError::NotFound(id))
    }
}

async fn delete_by_id(path: Path<(i32,)>, pool: Data<PgPool>) -> AppResult {
    let (id,) = path.into_inner();

    let todo = sqlx::query_as::<_, Todo>(
        r#"
            DELETE FROM todo
            WHERE id = $1
            RETURNING *
        "#
    )
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?;

    if let Some(_) = todo {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(id))
    }
}