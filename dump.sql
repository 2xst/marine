PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE users (
  id integer primary key autoincrement,
  email varchar(255) unique not null,
  password_hash varchar(255) not null
);
INSERT INTO users VALUES(1488,'grimerssy@gmail.com','$argon2id$v=19$m=16,t=1,p=1$c/feVbltmBkn5eHyGnEpxA$NiMcIgQGcUHd2GgWamjr8gGjznIc9PdaviKl7c6RvQc');
CREATE TABLE locations (
  id integer primary key autoincrement,
  country varchar(255) not null,
  city varchar(255) not null,
  address varchar(255) not null
, partner_id integer references partners(id));
INSERT INTO locations VALUES(100,'Croatia','Zagreb','Ilica 1',10);
INSERT INTO locations VALUES(101,'Germany','Kiel','Teliukstraße 52a',11);
INSERT INTO locations VALUES(102,'Poland','Gdansk','ul. Długa 1',11);
CREATE TABLE partners (
  id integer primary key autoincrement,
  name varchar(255) unique not null,
  password_hash varchar(255) not null
);
INSERT INTO partners VALUES(10,'Maelstrom','$argon2id$v=19$m=16,t=1,p=1$c/feVbltmBkn5eHyGnEpxA$NiMcIgQGcUHd2GgWamjr8gGjznIc9PdaviKl7c6RvQc');
INSERT INTO partners VALUES(11,'Typhoon','$argon2id$v=19$m=16,t=1,p=1$c/feVbltmBkn5eHyGnEpxA$NiMcIgQGcUHd2GgWamjr8gGjznIc9PdaviKl7c6RvQc');
CREATE TABLE records(
  id integer primary key autoincrement,
  date varchar(255) not null,
  depth integer not null,
  max_pressure integer not null,
  user_id integer not null references users(id),
  location_id integer not null references locations(id)
);
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('users',1);
INSERT INTO sqlite_sequence VALUES('locations',104);
INSERT INTO sqlite_sequence VALUES('partners',11);
COMMIT;
INSERT INTO records VALUES(100001,'2020-12-17',1, 101325,1488,101);
INSERT INTO records VALUES(100002,'2020-12-17',10, 135932,1488,101);
INSERT INTO records VALUES(100003,'2022-04-24',15, 151782,1488,103);
