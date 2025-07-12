-- Add up migration script here
CREATE TABLE user_auth_token (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    token VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);