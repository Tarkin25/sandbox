table! {
    author (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    author_book (id) {
        id -> Int4,
        author_id -> Int4,
        book_id -> Int4,
    }
}

table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
    }
}

joinable!(author_book -> author (author_id));
joinable!(author_book -> book (book_id));

allow_tables_to_appear_in_same_query!(
    author,
    author_book,
    book,
);
