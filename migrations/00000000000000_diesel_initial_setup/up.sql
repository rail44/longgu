-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included

--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE instances (
    id SERIAL PRIMARY KEY,
    domain VARCHAR,
    is_myself BOOLEAN,
    UNIQUE (domain)
);

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    instance_id INTEGER NOT NULL,
    user_name VARCHAR NOT NULL,
    display_name VARCHAR NOT NULL,
    UNIQUE (user_name, instance_id),
    FOREIGN KEY (instance_id) REFERENCES instances (id)
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    FOREIGN KEY (account_id) REFERENCES accounts (id)
);
