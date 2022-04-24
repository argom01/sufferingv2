CREATE TABLE users (
    id int NOT NULL AUTO_INCREMENT PRIMARY KEY,
    email varchar(255) NOT NULL,
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    is_admin boolean NOT NULL,
    token_version int DEFAULT 0 NOT NULL,
    CONSTRAINT UC_User UNIQUE (id, username, email)
);