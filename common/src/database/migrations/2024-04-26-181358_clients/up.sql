-- Your SQL goes here
CREATE TABLE Clients(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(90) NOT NULL,
    email VARCHAR(90) NULL,
    person_id VARCHAR(13) NULL,
    tel_num VARCHAR(15) NULL
)