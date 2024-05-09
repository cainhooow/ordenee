-- Your SQL goes here
CREATE TABLE Equipaments(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    serie VARCHAR(155) NULL UNIQUE,
    model VARCHAR(50) NOT NULL,
    description TEXT NULL,
    barcode INTEGER NULL
)