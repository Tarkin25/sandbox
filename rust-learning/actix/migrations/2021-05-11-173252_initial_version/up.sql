create table author (
    id serial primary key,
    name varchar not null
);

create table book (
    id serial primary key,
    title varchar not null
);

create table author_book (
    id serial primary key,
    author_id int not null,
    book_id int not null,
    constraint fk_author foreign key (author_id) references author (id),
    constraint fk_book foreign key (book_id) references book (id),
    constraint un_author_book unique (author_id, book_id)
);