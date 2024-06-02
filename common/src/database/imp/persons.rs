use std::vec;

use diesel::{
    prelude::Insertable, result::Error, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};

use serde::{Deserialize, Serialize};

use crate::database::{
    models::Persons,
    schema::{self},
    Database,
};

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::persons)]
pub struct PersonsBase<'r> {
    pub name: &'r str,
    pub email: Option<&'r str>,
    pub tel_num: Option<&'r str>,
    pub person_id: Option<&'r str>,
    pub is_technical: Option<bool>,
}

impl Persons {
    pub fn create(person: PersonsBase) -> Result<Persons, Error> {
        let database = Database::init();
        let mut connection = database.connection;

        use self::schema::persons::dsl::*;
        use self::schema::persons::*;

        match diesel::insert_into(persons)
            .values(vec![&person])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:client:create()");

                let last_person = persons
                    .select(id)
                    .order(id.desc())
                    .first::<i32>(&mut connection)?;

                let person = Self::person(last_person)?;

                Ok(person)
            }
            Err(err) => {
                println!(":ORDENNE:database:client:create() exception: {:?}", err);
                Err(err)
            }
        }
    }
    pub fn person(person: i32) -> Result<Persons, Error> {
        use self::schema::persons::dsl::*;
        use self::schema::persons::*;

        let database = Database::init();
        let mut connection = database.connection;

        match persons
            .filter(id.eq(person))
            .select(Persons::as_select())
            .get_result(&mut connection)
        {
            Ok(person) => Ok(person),
            Err(err) => Err(err),
        }
    }

    pub fn all() -> Result<Vec<Persons>, Error> {
        use self::schema::persons;

        let database = Database::init();
        let mut connection = database.connection;

        match persons::table.get_results::<Persons>(&mut connection) {
            Ok(res) => {
                println!(":ORDENNE:database:client:all() {:?}", res);
                Ok(res)
            }
            Err(err) => {
                println!(":ORDENNE:database:client:all() exception: {:?}", err);
                Err(err)
            }
        }
    }
}
