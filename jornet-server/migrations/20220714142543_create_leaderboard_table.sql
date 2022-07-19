CREATE TABLE leaderboards(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    owner UUID NOT NULL
);
