
use std::collections::HashMap;
use actix_web::web::{ServiceConfig, Data};
use crate::db::Pool;
use actix_web::{Responder, HttpResponse};
use diesel::prelude::*;
use crate::models::{Author, Book};
use crate::schema::{author, book, author_book};
use serde::{Serialize};

pub fn configure_routes(config: &mut ServiceConfig) {
    use actix_web::web::*;

    config
        .service(
            resource("/authors")
            .route(get().to(find_all))
        );
}

#[derive(Serialize)]
struct AuthorWithBooks {
    id: i32,
    name: String,
    books: Vec<Book>,
}

impl AuthorWithBooks {
    fn new(tuple: (Author, Vec<Book>)) -> Self {
        let (Author {id, name}, books) = tuple;

        AuthorWithBooks {
            id,
            name,
            books,
        }
    }
}

async fn find_all(pool: Data<Pool>) -> impl Responder {
    let conn = pool.get().unwrap();

    let query = author::table
    .inner_join(author_book::table.inner_join(book::table))
    .select((author::all_columns, book::all_columns));

    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query);

    log::debug!("{:?}", sql);

    let result = query
    .load::<(Author, Book)>(&conn)
    .unwrap();

    let mut map: HashMap<i32, (Author, Vec<Book>)> = HashMap::new();

    for (author, book) in result.into_iter() {
        let (_, books) = map.entry(author.id).or_insert((author, Vec::new()));

        books.push(book);
    }

    let authors: Vec<AuthorWithBooks> = map.values().cloned().map(AuthorWithBooks::new).collect();

    HttpResponse::Ok().json(authors)
}