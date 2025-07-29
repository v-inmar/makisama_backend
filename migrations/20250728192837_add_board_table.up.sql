-- Add up migration script here

CREATE TABLE board_pid(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(32) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE board_title(
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE board (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    creator_id BIGINT NOT NULL,
    title_id BIGINT NOT NULL,
    board_pid_id BIGINT NOT NULL,
    FOREIGN KEY (title_id) REFERENCES board_title(id),
    FOREIGN KEY (creator_id) REFERENCES user(id),
    FOREIGN KEY (board_pid_id) REFERENCES board_pid(id)
);

CREATE TABLE board_creator (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES board(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE board_member (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES board(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE board_admin (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES board(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

