use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{AppError, AppResult};
use std::ops::Deref;

mod repository;

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
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    title: String,
    description: Option<String>,
    completed: bool,
}

async fn find_all(pool: Data<PgPool>) -> AppResult {
    let todos = repository::find_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(todos))
}

async fn find_by_id(path: Path<(i32, )>, pool: Data<PgPool>) -> AppResult {
    let (id, ) = path.into_inner();

    let todo = repository::find_by_id(id, pool.get_ref()).await?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        Err(AppError::NotFound(id))
    }
}

async fn create(todo: Json<CreateTodo>, pool: Data<PgPool>) -> AppResult {
    let todo = repository::insert(todo.deref(), pool.get_ref()).await?;

    Ok(HttpResponse::Created().json(todo))
}

async fn update_by_id(path: Path<(i32, )>, todo: Json<UpdateTodo>, pool: Data<PgPool>) -> AppResult {
    let (id,) = path.into_inner();

    let todo = repository::update_by_id(id, todo.deref(), pool.get_ref()).await?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        Err(AppError::NotFound(id))
    }
}

async fn delete_by_id(path: Path<(i32,)>, pool: Data<PgPool>) -> AppResult {
    let (id,) = path.into_inner();

    let todo = repository::delete_by_id(id, pool.get_ref()).await?;

    if let Some(_) = todo {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(id))
    }
}