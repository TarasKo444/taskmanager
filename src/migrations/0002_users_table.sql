create table users
(
    id            varchar not null,
    username      varchar not null,
    email         varchar not null,
    picture       varchar not null,
    joined_at     TIMESTAMPTZ not null,
    refresh_token varchar not null,
    PRIMARY KEY (id)
);
