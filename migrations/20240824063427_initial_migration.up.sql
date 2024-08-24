-- This is the initial migration setting up a framework for basic user account management and
-- authentication.

-- The `citext` extension is used for case-insensitive email uniqueness checks.
CREATE EXTENSION IF NOT EXISTS citext;

-- An incomplete user account table which the below tables reference. This table will be altered
-- such as to reference the current active email address to use in communications.
CREATE TABLE user_account (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,

    -- The name to refer to the user as in communications and in UI
    display_name TEXT NOT NULL,

    -- Salted/hashed password using Argon2.
    password_hash TEXT NOT NULL
);

-- User email addresses are stored in a separate table such as to allow associating multiple emails
-- with one account.
CREATE TABLE user_email_address (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INT REFERENCES user_account(id) NOt NULL,
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,

    email_address CITEXT UNIQUE NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE
);

-- Users will additionally be able to have one or more API keys associated with them. However, at
-- present, this will be used only for an ephemeral session token for use with mutating account
-- information once logged in.
CREATE TABLE user_api_key (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INT REFERENCES user_account(id) NOt NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    
    api_key TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

-- Update the `user_account` table to denote which email address to use as the primary means of
-- communications.
ALTER TABLE user_account ADD COLUMN primary_email INT REFERENCES user_email_address(id) NOT NULL;
