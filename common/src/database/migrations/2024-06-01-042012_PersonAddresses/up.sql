-- Your SQL goes here
CREATE TABLE PersonAddresses(
    id INT PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    home_num INT NULL,
    street TEXT NULL,
    city TEXT NULL,
    person_id INT NOT NULL,

    FOREIGN KEY (person_id) REFERENCES Persons(id)
)