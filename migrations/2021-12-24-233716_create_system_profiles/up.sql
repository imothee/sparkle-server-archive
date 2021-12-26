CREATE TABLE system_profiles (
                          id SERIAL PRIMARY KEY,
                          app_id INT NOT NULL,
                          app_version VARCHAR,
                          cpu64bit BOOLEAN,
                          ncpu INTEGER,
                          cpu_freq_mhz VARCHAR,
                          cputype VARCHAR,
                          cpusubtype VARCHAR,
                          model VARCHAR,
                          ram_mb VARCHAR,
                          os_version VARCHAR,
                          lang VARCHAR,
                          created_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                          updated_at TIMESTAMP WITH TIME ZONE default CURRENT_TIMESTAMP NOT NULL,
                          CONSTRAINT fk_app
                              FOREIGN KEY(app_id)
                                  REFERENCES apps(id)
);

CREATE INDEX ON system_profiles (app_id);
SELECT diesel_manage_updated_at('system_profiles');