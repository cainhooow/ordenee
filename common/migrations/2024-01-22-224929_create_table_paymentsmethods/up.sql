-- Your SQL goes here
CREATE TABLE IF NOT EXISTS PaymentMethods(
    pay_method_id INT PRIMARY KEY NOT NULL AUTOINCREMENT COMMENT("Pay method indentifier"),
    pay_method_name VARCHAR(50) NOT NULL COMMENT("Pay method name"), 
    pay_method_description VARCHAR(250) NULL COMMENT("Pay method description. Not required")
)