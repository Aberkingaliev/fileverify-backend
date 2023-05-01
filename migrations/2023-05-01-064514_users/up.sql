CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    is_activated BOOLEAN NOT NULL DEFAULT FALSE,
    activation_link VARCHAR NOT NULL UNIQUE
); 