use std::vec;

use diesel::{prelude::Insertable, RunQueryDsl};

use crate::database::{models::Clients, schema, Database};

#[derive(Insertable)]
#[diesel(table_name = schema::clients)]
pub struct ClientBase<'r> {
    pub name: &'r str,
}

impl Clients {
    pub fn create(client: ClientBase) {
        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(schema::clients::table)
            .values(vec![&client])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:client:create()");
            }
            Err(err) => {
                println!(":ORDENNE:database:client:create() exception: {:?}", err);
            }
        }
    }
    pub fn find() {}

    pub fn all() {}
}
