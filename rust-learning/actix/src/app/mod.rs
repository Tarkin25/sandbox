use actix_web::web::ServiceConfig;

mod author;
mod book;
mod author_book;

pub fn configure_routes(config: &mut ServiceConfig) {
    author::configure_routes(config);
    book::configure_routes(config);
    author_book::configure_routes(config);
}