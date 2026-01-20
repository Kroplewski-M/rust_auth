-- Add down migration script here
ALTER TABLE users
ALTER COLUMN created_at SET NULL,
ALTER COLUMN updated_at SET NULL;
