CREATE DATABASE orders;
CREATE USER orders WITH PASSWORD 'order123';

\c orders

CREATE TABLE client (
	id SERIAL PRIMARY KEY,
	name VARCHAR(64) NOT NULL,
	email VARCHAR(64) NOT NULL,
    comuna VARCHAR(64) NOT NULL,
    direccion VARCHAR(64) NOT NULL,
    depto VARCHAR(16)
);

CREATE TABLE orders (
	id SERIAL PRIMARY KEY,
    client INT NOT NULL,
	product VARCHAR(16) NOT NULL,
	qty INT NOT NULL,
	status VARCHAR(16) NOT NULL DEFAULT 'Initial',
    FOREIGN KEY (client) REFERENCES client(id)
);