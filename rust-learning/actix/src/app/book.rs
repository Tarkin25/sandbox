use std::collections::HashMap;
use crate::models::Author;
use actix_web::web::{ServiceConfig, Data};
use crate::db::Pool;
use actix_web::{Responder, HttpResponse};
use diesel::prelude::*;
use crate::models::Book;
use serde::{Serialize};

pub fn configure_routes(config: &mut ServiceConfig) {
    use actix_web::web::*;

    config
        .service(
          resource("/books")
          .route(get().to(find_all))
        );
}

#[derive(Serialize)]
struct BookWithAuthors {
    id: i32,
    title: String,
    authors: Vec<Author>,
}

impl BookWithAuthors {
    fn new(tuple: (Book, Vec<Author>)) -> Self {
        let (Book { id, title }, authors) = tuple;

        BookWithAuthors {
            id,
            title,
            authors,
        }
    }
}

async fn find_all(pool: Data<Pool>) -> impl Responder {
    use crate::schema::book::dsl as book_dsl;
    use crate::schema::author_book::dsl as author_book_dsl;
    use crate::schema::author::dsl as author_dsl;

    let conn = pool.get().unwrap();

    let query = book_dsl::book
    .inner_join(author_book_dsl::author_book.inner_join(author_dsl::author))
    .select((book_dsl::book::all_columns(), author_dsl::author::all_columns()));

    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    log::debug!("{:?}", sql);

    let result = query
    .load::<(Book, Author)>(&conn)
    .unwrap();

    let mut map: HashMap<i32, (Book, Vec<Author>)> = HashMap::new();

    for (book, author) in result.into_iter() {
        let (_, authors) = map.entry(book.id).or_insert((book, Vec::new()));

        authors.push(author);
    }

    let books: Vec<BookWithAuthors> = map.values().cloned().map(BookWithAuthors::new).collect();

    HttpResponse::Ok().json(books)
}