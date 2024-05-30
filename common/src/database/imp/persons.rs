use std::vec;

use diesel::{prelude::Insertable, result::Error, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::database::{models::Persons, schema, Database};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::persons)]
pub struct PersonsBase<'r> {
    pub name: &'r str,
    pub email: Option<&'r str>,
    pub tel_num: Option<&'r str>,
    pub person_id: Option<&'r str>,
    pub is_technical: Option<bool>
}

impl Persons {
    pub fn create(person: PersonsBase) -> Result<(), Error> {
        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(schema::persons::table)
            .values(vec![&person])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:client:create()");
                Ok(())
            }
            Err(err) => {
                println!(":ORDENNE:database:client:create() exception: {:?}", err);
                Err(err)
            }
        }
    }
    pub fn find() {}

    pub fn all() {
        use self::schema::persons;

        let database = Database::init();
        let mut connection = database.connection;

        match persons::table.get_results::<Persons>(&mut connection) {
            Ok(res) => {
                println!(":ORDENNE:database:client:all() {:?}", res);
            }
            Err(err) => {
                println!(":ORDENNE:database:client:all() exception: {:?}", err);
            }
        }
    }
}
