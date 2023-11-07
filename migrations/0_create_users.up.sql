create table users(
    id int unsigned auto_increment primary key,
    email varchar(50) not null unique,
    password_hash char(100) not null,
    verification_token char(32) not null unique,
    verified boolean not null,
    refresh_token char(32)
);
