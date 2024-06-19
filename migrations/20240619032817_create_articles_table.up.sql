-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS `articles` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `author_id` BIGINT UNSIGNED NULL,
    `title` VARCHAR(100) NOT NULL,
    `description` VARCHAR(255) NULL,
    `markdown_body` TEXT NULL,
    `status` TINYINT(1) NOT NULL,
    `created_at` TIMESTAMP NULL,
    `updated_at` TIMESTAMP NULL,
    PRIMARY KEY (`id`)
);