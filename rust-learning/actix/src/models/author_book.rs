use crate::schema::author_book;
use diesel::{Queryable, Identifiable, Associations};
use serde::{Serialize};
use super::{Author, Book};

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Author)]
#[belongs_to(Book)]
#[table_name="author_book"]
pub struct AuthorBook {
    id: i32,
    author_id: i32,
    book_id: i32,
}