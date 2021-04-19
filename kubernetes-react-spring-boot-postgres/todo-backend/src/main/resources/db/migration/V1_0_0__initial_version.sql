create table todo (
    id serial,
    title varchar(255) not null,
    description varchar(255),
    completed boolean not null default false,
    constraint pk_todo primary key (id)
);