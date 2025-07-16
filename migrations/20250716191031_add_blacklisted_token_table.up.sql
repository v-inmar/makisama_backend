-- Add up migration script here
CREATE TABLE blacklisted_token (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value TEXT NOT NULL,
    datetime_ttl TIMESTAMP NOT NULL,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);