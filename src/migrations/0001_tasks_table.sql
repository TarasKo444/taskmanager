create table tasks
(
    id          uuid DEFAULT gen_random_uuid(),
    title       varchar not null,
    description varchar not null,
    created_at  TIMESTAMPTZ not null,
    PRIMARY KEY (id)
);
