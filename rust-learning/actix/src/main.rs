#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{App, HttpServer};
use dotenv;

mod app;
mod db;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix=debug");
    env_logger::init();

    dotenv::dotenv().ok();

    let pool = db::create_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(app::configure_routes)
    })
    .bind("127.0.0.1:7878")?
    .bind("10.0.1.122:7878")?
    .bind("localhost:7878")?
    .run()
    .await
}
