CREATE TABLE metrics (
                                 id SERIAL PRIMARY KEY,
                                 app_id INT NOT NULL,
                                 date DATE NOT NULL,
                                 profile_key VARCHAR NOT NULL,
                                 profile_value VARCHAR NOT NULL,
                                 count INTEGER NOT NULL,
                                 created_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                                 updated_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                                 CONSTRAINT fk_app
                                     FOREIGN KEY(app_id)
                                         REFERENCES apps(id)
);

CREATE INDEX ON metrics (app_id);
SELECT diesel_manage_updated_at('metrics');