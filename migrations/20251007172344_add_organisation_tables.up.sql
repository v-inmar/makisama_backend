-- Add up migration script here
CREATE TABLE organisation_pid (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value VARCHAR(64) NOT NULL UNIQUE -- randomly generated string
);

CREATE TABLE organisation (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name VARCHAR(128) NOT NULL UNIQUE,
    description VARCHAR(255) NULL,
    datetime_deleted DATETIME DEFAULT NULL,                    
    pid_id BIGINT UNSIGNED NOT NULL UNIQUE,             
    PRIMARY KEY (id),
    FOREIGN KEY (pid_id) REFERENCES organisation_pid(id)                    
);


CREATE TABLE organisation_member (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    organisation_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    is_owner TINYINT(1) NOT NULL DEFAULT 0,
    is_admin TINYINT(1) NOT NULL DEFAULT 0,
    datetime_removed DATETIME DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (organisation_id) REFERENCES organisation(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);