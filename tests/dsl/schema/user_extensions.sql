DROP TABLE IF EXISTS user_profiles CASCADE;
DROP TABLE IF EXISTS user_settings CASCADE;

CREATE TABLE user_profiles (
    user_id int8 PRIMARY KEY REFERENCES users (id),
    bio     text NOT NULL
);

CREATE TABLE user_settings (
    user_id int8 PRIMARY KEY REFERENCES users (id),
    theme   text NOT NULL
);
