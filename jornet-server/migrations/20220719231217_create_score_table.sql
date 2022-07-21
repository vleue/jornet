CREATE TABLE scores(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    leaderboard UUID NOT NULL,
    score REAL NOT NULL,
    player UUID NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    meta TEXT
);
