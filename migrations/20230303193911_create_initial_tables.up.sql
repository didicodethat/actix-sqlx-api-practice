-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    phone_number VARCHAR(256) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    user_id int NOT NULL,
    nickname VARCHAR(512) NOT NULL,
    confirmation_token VARCHAR(256) NOT NULL,
    confirmed_at TIMESTAMP,
    
    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);