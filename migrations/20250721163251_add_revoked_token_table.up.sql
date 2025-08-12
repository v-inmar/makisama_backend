-- Add up migration script here
CREATE TABLE revoked_token (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    value TEXT NOT NULL,
    datetime_ttl DATETIME NOT NULL,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);