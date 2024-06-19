-- Add up migration script here
ALTER TABLE `articles` ADD COLUMN `markdown_file` VARCHAR(255) DEFAULT NULL AFTER `markdown_body`;