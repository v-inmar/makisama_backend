-- Add up migration script here
-- Drop the foreign key constraint for `username_id`
ALTER TABLE `user`
DROP FOREIGN KEY `user_ibfk_2`;

-- Drop the `username_id` column
ALTER TABLE `user`
DROP COLUMN `username_id`;

-- Drop the username table
DROP TABLE username;

