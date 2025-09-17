CREATE TABLE
    IF NOT EXISTS user (
        email TEXT NOT NULL PRIMARY KEY,
        password_hash TEXT NOT NULL,
        requires_2fa BOOLEAN NOT NULL DEFAULT FALSE
    );