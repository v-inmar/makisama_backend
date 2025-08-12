-- Add up migration script here
CREATE TABLE board_name (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value VARCHAR(128) NOT NULL UNIQUE
);

CREATE TABLE board_pid (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE board_description (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);

CREATE TABLE board (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,  
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,          
    datetime_deleted DATETIME NULL,                       
    pid_id BIGINT UNSIGNED NOT NULL UNIQUE,             
    name_id BIGINT UNSIGNED NOT NULL,            
    description_id BIGINT UNSIGNED NULL,     
    PRIMARY KEY (id),
    FOREIGN KEY (name_id) REFERENCES board_name(id),
    FOREIGN KEY (pid_id) REFERENCES board_pid(id),
    FOREIGN KEY (description_id) REFERENCES board_description(id)                             
);


CREATE TABLE board_user (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT, 
    datetime_created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,  
    board_id BIGINT UNSIGNED NOT NULL,  
    user_id BIGINT UNSIGNED NOT NULL,  
    is_owner TINYINT(1) NOT NULL DEFAULT 0,  
    is_admin TINYINT(1) NOT NULL DEFAULT 0,
    datetime_removed DATETIME NULL,  
    PRIMARY KEY (id), 
    FOREIGN KEY (board_id) REFERENCES board(id),  
    FOREIGN KEY (user_id) REFERENCES user(id)  
);





