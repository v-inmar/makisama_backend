-- Add up migration script here
CREATE TABLE auth_identity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_ttl TIMESTAMP DEFAULT NULL,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE username (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE firstname (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE lastname (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `user` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    username VARCHAR(64) NOT NULL UNIQUE,
    datetime_deactivated TIMESTAMP DEFAULT NULL,
    datetime_deleted TIMESTAMP DEFAULT NULL,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    auth_identity_id BIGINT NOT NULL UNIQUE,
    username_id BIGINT NOT NULL UNIQUE,
    firstname_id BIGINT NOT NULL,
    lastname_id BIGINT NOT NULL,
    FOREIGN KEY (auth_identity_id) REFERENCES auth_identity(id),
    FOREIGN KEY (username_id) REFERENCES username(id),
    FOREIGN KEY (firstname_id) REFERENCES firstname(id),
    FOREIGN KEY (lastname_id) REFERENCES lastname(id)
);