use std::env;
use std::error::Error;

use actix_web::{App, HttpServer};

use todo_backend::db;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "actix_web=info,todo_backend=debug");
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = db::create_pool().await?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(todo_backend::configure_routes)
    })
        .bind("127.0.0.1:7878")?
        .bind("10.0.1.122:7878")?
        .bind("localhost:7878")?
        .run()
        .await?;

    Ok(())
}
