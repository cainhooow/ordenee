-- Your SQL goes here
CREATE TABLE PaymentMethods(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(10) NOT NULL UNIQUE,
    created_at DATE NOT NULL DEFAULT (DATETIME(CURRENT_TIMESTAMP, 'localtime')),
    updated_at DATE NULL
)