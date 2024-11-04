CREATE TABLE IF NOT EXISTS greetings
(
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    greeting   VARCHAR
);

CREATE TABLE IF NOT EXISTS users
(
    id         BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE TYPE task_status AS ENUM ('created', 'inprogress', 'completed');

CREATE TABLE IF NOT EXISTS tasks
(
    id          BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    created_at  TIMESTAMPTZ                  NOT NULL,
    updated_at  TIMESTAMPTZ                  NOT NULL,
    user_id     BIGINT REFERENCES users (id) NOT NULL,
    name        VARCHAR                      NOT NULL,
    description VARCHAR,
    status      task_status                  NOT NULL
);

CREATE TABLE IF NOT EXISTS tags
(
    id   BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    name VARCHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks_tags
(
    id      BIGINT UNIQUE GENERATED ALWAYS AS IDENTITY,
    task_id BIGINT REFERENCES tasks (id) NOT NULL,
    tag_id  BIGINT REFERENCES tags (id)  NOT NULL
);
