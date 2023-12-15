create table users (
  id integer primary key autoincrement,
  email varchar(255) unique not null,
  password_hash varchar(255) not null
);
