-- Add down migration script here
-- Recreate the `username` table
CREATE TABLE username (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(64) NOT NULL UNIQUE,
    datetime_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add the `username_id` column back to the `user` table
ALTER TABLE `user`
ADD COLUMN `username_id` BIGINT NOT NULL UNIQUE;

-- Add the foreign key constraint back for `username_id`
ALTER TABLE `user`
ADD CONSTRAINT `user_ibfk_2`
    FOREIGN KEY (`username_id`) REFERENCES `username`(`id`);