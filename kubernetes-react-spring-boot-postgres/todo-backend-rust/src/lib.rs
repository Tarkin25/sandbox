use actix_web::web::ServiceConfig;

pub use error::*;
use actix_web::HttpResponse;

mod todo;
mod error;

pub type AppResult = Result<HttpResponse, AppError>;

pub mod db {
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;

    pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost:5432/todo")
            .await
    }
}

pub fn configure_routes(config: &mut ServiceConfig) {
    todo::configure_routes(config);
}

