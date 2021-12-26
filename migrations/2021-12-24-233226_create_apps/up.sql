CREATE TABLE apps (
                       id SERIAL PRIMARY KEY,
                       name VARCHAR NOT NULL,
                       slug VARCHAR NOT NULL,
                       description VARCHAR NOT NULL,
                       icon VARCHAR,
                       created_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                       updated_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX ON apps (slug);
SELECT diesel_manage_updated_at('apps');