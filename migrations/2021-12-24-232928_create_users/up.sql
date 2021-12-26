CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       email VARCHAR NOT NULL,
                       password_token VARCHAR NOT NULL,
                       last_login TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                       created_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX ON users (email);
SELECT diesel_manage_updated_at('users');