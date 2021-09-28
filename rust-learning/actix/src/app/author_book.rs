use crate::db::Pool;
use crate::models::{Book};
use crate::schema::book::dsl::book;
use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{HttpResponse, Responder};
use diesel::prelude::*;

pub fn configure_routes(config: &mut ServiceConfig) {
    use actix_web::web::*;

    config
        .service(
            resource("/authors/{id}/books")
            .route(get().to(find_all_by_author_id))
        );
}

async fn find_all_by_author_id(path: Path<(i32,)>, pool: Data<Pool>) -> impl Responder {
    use crate::schema::author_book::dsl::*;

    let Path((_id,)) = path;

    let connection = pool.get().unwrap();

    let result = author_book
        .filter(author_id.eq(_id))
        .inner_join(book)
        .select(book::all_columns())
        .load::<Book>(&connection)
        .unwrap();

    HttpResponse::Ok().json(result)
}
