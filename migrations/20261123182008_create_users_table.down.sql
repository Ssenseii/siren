-- Add down migration script here
DROP INDEX IF EXISTS "idx_users_email";
DROP INDEX IF EXISTS "idx_users_username";
DROP TABLE IF EXISTS "users";