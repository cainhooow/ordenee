use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Clients {
    pub id: i32,
    pub name: String,
    pub person_id: String,
    pub tel_num: Option<String>,
}