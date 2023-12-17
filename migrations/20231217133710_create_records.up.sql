create table records(
  id integer primary key autoincrement,
  date varchar(255) not null,
  depth integer not null,
  max_pressure integer not null,
  user_id integer not null references users(id),
  location_id integer not null references locations(id)
);
