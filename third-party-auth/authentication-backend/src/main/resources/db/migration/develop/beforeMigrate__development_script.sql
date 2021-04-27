drop table if exists user_role;
drop table if exists role_authority;
drop table if exists role;
drop table if exists authority;
drop table if exists users;

create table users
(
    id        serial
        constraint pk_users primary key,
    google_id varchar(255),
    facebook_id varchar(255),
    email     varchar(255) not null,
    name      varchar(255)
);

create table authority
(
    id serial constraint pk_authority primary key,
    name varchar(255) not null
);

create table role
(
    id serial constraint pk_role primary key,
    name varchar(255) not null
);

create table role_authority
(
    role_id int not null,
    authority_id int not null,
    constraint fk_role_authority_role foreign key (role_id) references role (id),
    constraint fk_role_authority_authority foreign key (authority_id) references authority (id),
    constraint role_id_authority_id unique (role_id, authority_id)
);

create table user_role
(
    user_id int not null,
    role_id int not null,
    constraint fk_user_role_user foreign key (user_id) references users (id),
    constraint fk_user_role_role foreign key (role_id) references role (id),
    constraint user_id_role_id unique (user_id, role_id)
);