--Bsae app schema

--User

CREATE TABLE IF NOT EXISTS "user" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  username varchar(128) not null unique
);


--Task

CREATE TABLE IF NOT EXISTS task (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
  title  varchar(256) NOT NULL
);