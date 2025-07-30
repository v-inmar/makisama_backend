-- Add up migration script here
CREATE TABLE board_name (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_lowercase CHECK (BINARY value = LOWER(value))
);

CREATE TABLE board (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_name_id BIGINT NOT NULL,
    FOREIGN KEY (board_name_id) REFERENCES board_name(id)
);

CREATE TABLE board_member (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    is_owner TINYINT(1) NOT NULL DEFAULT 0,
    is_admin TINYINT(1) NOT NULL DEFAULT 0,
    FOREIGN KEY (board_id) REFERENCES board(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);
