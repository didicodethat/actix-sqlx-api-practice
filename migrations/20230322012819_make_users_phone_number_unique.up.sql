-- Add up migration script here
ALTER TABLE users
ADD CONSTRAINT unique_users_phone_number UNIQUE (phone_number);