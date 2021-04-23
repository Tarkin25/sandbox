create table users
(
    id        serial
        constraint pk_users primary key,
    google_id varchar(255) not null
        constraint users_google_id unique,
    email     varchar(255) not null,
    name      varchar(255),
    deleted   boolean      not null default false
);