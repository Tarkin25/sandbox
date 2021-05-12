use crate::schema::author;
use diesel::{Queryable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Queryable, Identifiable)]
#[table_name="author"]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name="author"]
pub struct NewAuthor {
    pub name: String,
}