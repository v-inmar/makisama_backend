-- Add up migration script here
CREATE TABLE `user` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    datetime_deactivated TIMESTAMP DEFAULT NULL,
    datetime_deleted TIMESTAMP DEFAULT NULL,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);