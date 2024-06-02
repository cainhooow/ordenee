-- Your SQL goes here
CREATE TABLE PersonAddresses(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    home_num INT NULL,
    street TEXT NULL,
    city TEXT NULL,
    person_id INT NOT NULL,
    created_at DATE NOT NULL DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'localtime')),
    updated_at DATE NULL,
    FOREIGN KEY (person_id) REFERENCES Persons(id)
)