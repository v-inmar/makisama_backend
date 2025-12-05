-- Add up migration script here
CREATE TABLE user_authid (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_name (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_email (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user_pid (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `user` (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    password VARCHAR(255) NOT NULL,
    datetime_confirmed DATETIME DEFAULT NULL,
    datetime_deactivated DATETIME DEFAULT NULL,
    datetime_deleted DATETIME DEFAULT NULL,
    authid_id BIGINT NOT NULL UNIQUE,
    email_id BIGINT NOT NULL UNIQUE,
    pid_id BIGINT NOT NULL UNIQUE,
    firstname_id BIGINT NOT NULL,
    lastname_id BIGINT NOT NULL,
    FOREIGN KEY (authid_id) REFERENCES user_authid(id),
    FOREIGN KEY (email_id) REFERENCES user_email(id),
    FOREIGN KEY (pid_id) REFERENCES user_pid(id),
    FOREIGN KEY (firstname_id) REFERENCES user_name(id),
    FOREIGN KEY (lastname_id) REFERENCES user_name(id)
);