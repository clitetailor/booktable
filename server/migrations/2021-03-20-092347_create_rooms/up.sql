-- Your SQL goes here
CREATE TABLE rooms (
  id serial PRIMARY KEY,
  name VARCHAR ( 100 ) UNIQUE NOT NULL,
  capacity INTEGER NOT NULL CHECK ( capacity >= 0 )
);
