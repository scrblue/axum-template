-- This reverses the initial migration for setting up account management and authentication.

ALTER TABLE user_account DROP COLUMN primary_email;

DROP TABLE user_api_key;
DROP TABLE user_email_address;
DROP TABLE user_account;

DROP EXTENSION citext;
