use crate::schema::book;
use diesel::{Queryable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Queryable, Identifiable)]
#[table_name="book"]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name="book"]
pub struct NewBook {
    pub title: String,
}
