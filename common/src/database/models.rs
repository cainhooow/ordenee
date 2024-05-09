use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = super::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Clients {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub person_id: Option<String>,
    pub tel_num: Option<String>
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = super::schema::equipaments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equipaments {
    pub id: i32,
    pub name: String,
    pub serie: Option<String>,
    pub model: String,
    pub description: Option<String>,
    pub barcode: Option<i32>
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = super::schema::paymentmethods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PaymentMethods {
    pub id: i32,
    pub name: String
}