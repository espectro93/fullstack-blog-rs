CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    id            UUID                                NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at    TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at    TIMESTAMP DEFAULT current_timestamp NOT NULL,
    email         VARCHAR(120) UNIQUE                 NOT NULL,
    password_hash BYTEA                               NOT NULL,
    last_action   TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

CREATE UNIQUE INDEX email_idx ON users (email);

CREATE TABLE posts
(
    id           UUID                                NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID                                NOT NULL REFERENCES users (id),
    title        TEXT                                NOT NULL,
    created_at   TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at   TIMESTAMP DEFAULT current_timestamp NOT NULL,
    body         TEXT                                NOT NULL,
    status       TEXT                                NOT NULL,
    published_at TIMESTAMP
);

SELECT diesel_manage_updated_at('posts');

CREATE TABLE comments
(
    id         UUID                                NOT NULL PRIMARY KEY,
    user_id    UUID                                NOT NULL REFERENCES users (id),
    post_id    UUID                                NOT NULL REFERENCES posts (id),
    body       TEXT                                NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);

SELECT diesel_manage_updated_at('comments');