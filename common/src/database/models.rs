use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = super::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Persons {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub person_id: Option<String>,
    pub tel_num: Option<String>,
    pub is_technical: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(
    Associations, Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize,
)]
#[diesel(table_name = super::schema::personaddresses)]
#[diesel(belongs_to(Persons, foreign_key = person_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PersonAddresses {
    pub id: i32,
    pub address: String,
    pub home_num: Option<i32>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub person_id: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
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
    pub barcode: Option<i32>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = super::schema::paymentmethods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PaymentMethods {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
