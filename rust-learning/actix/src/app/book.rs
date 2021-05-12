use actix_web::web::{ServiceConfig, Data};
use crate::db::Pool;
use actix_web::{Responder, HttpResponse};
use diesel::prelude::*;
use crate::models::Book;

pub fn configure_routes(config: &mut ServiceConfig) {
    use actix_web::web::*;

    config
        .service(
          scope("/books")
              .service(
                  resource("")
                      .route(get().to(find_all))
              )
        );
}

async fn find_all(pool: Data<Pool>) -> impl Responder {
    use crate::schema::book::dsl::*;

    let connection = pool.get().unwrap();

    let books = book.get_results::<Book>(&connection)
        .expect("Error retrieving books");

    HttpResponse::Ok().json(books)
}