create table partners (
  id integer primary key autoincrement,
  name varchar(255) unique not null,
  password_hash varchar(255) not null
);

alter table locations add column partner_id integer references partners(id);
