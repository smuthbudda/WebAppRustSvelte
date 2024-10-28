CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    active BOOLEAN,
    date_modified TIMESTAMP DEFAULT NULL,
    password VARCHAR
);

CREATE TABLE IF NOT EXISTS points (
    id SERIAL PRIMARY KEY,
    points INTEGER,
    gender VARCHAR(10),
    category VARCHAR(20),
    event VARCHAR(20),
    mark FLOAT,
    mark_time TIME DEFAULT '00:00:00'
);

CREATE TABLE IF NOT EXISTS user_points (
    user_id INTEGER REFERENCES users(id),
    point_id INTEGER REFERENCES points(id),
    PRIMARY KEY (user_id, point_id)
);

CREATE TABLE IF NOT EXISTS pages (
    id SERIAL PRIMARY KEY,
    text VARCHAR(255) NOT NULL,
    done BOOLEAN DEFAULT false NOT NULL
);
