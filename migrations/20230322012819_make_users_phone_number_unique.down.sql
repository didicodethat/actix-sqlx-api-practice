-- Add down migration script here
ALTER TABLE users
DROP CONSTRAINT unique_users_phone_number;
