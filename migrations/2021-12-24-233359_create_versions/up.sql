CREATE TABLE versions (
                      id SERIAL PRIMARY KEY,
                      app_id INT NOT NULL,
                      version VARCHAR NOT NULL,
                      min_system_version VARCHAR NOT NULL,
                      description VARCHAR NOT NULL,
                      url VARCHAR NOT NULL,
                      dsa_signature VARCHAR,
                      ed_signature VARCHAR,
                      length VARCHAR NOT NULL,
                      created_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                      updated_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                      CONSTRAINT fk_app
                        FOREIGN KEY(app_id)
                            REFERENCES apps(id)
);

CREATE INDEX ON versions (app_id);
SELECT diesel_manage_updated_at('versions');