-- Your SQL goes here
CREATE TABLE Persons(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(90) NOT NULL,
    email VARCHAR(90) NULL UNIQUE,
    person_id VARCHAR(13) NULL UNIQUE,
    tel_num VARCHAR(15) NULL,
    is_technical BOOLEAN NOT NULL DEFAULT false,
    created_at DATE NOT NULL DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'localtime')),
    updated_at DATE NULL
)