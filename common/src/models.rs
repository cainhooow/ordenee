use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Clients {
    pub id: i32,
    pub name: Option<String>,
    pub person_id: String,
    pub tel_num: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::equipaments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Equipaments {
    pub id: i32,
    pub name: Option<String>,
    pub serie: String,
    pub model: Option<String>,
    pub description: String,
    pub barcode: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::paymentmethods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PaymentMethods {
    pub id: i32,
    pub name: Option<String>
}