use std::vec;

use diesel::{
    prelude::Insertable, result::Error, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use serde::{Deserialize, Serialize};

use crate::database::{
    models::Persons,
    schema::{self, persons},
    Database,
};

use crate::database::models::PersonAddresses;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::persons)]
pub struct PersonsBase<'r> {
    pub name: &'r str,
    pub email: Option<&'r str>,
    pub tel_num: Option<&'r str>,
    pub person_id: Option<&'r str>,
    pub is_technical: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnableUser {
    pub person: Persons,
    pub address: Vec<PersonAddresses>,
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

                let person = Self::person(last_person)?.person;

                Ok(person)
            }
            Err(err) => {
                println!(":ORDENNE:database:client:create() exception: {:?}", err);
                Err(err)
            }
        }
    }
    pub fn person(person: i32) -> Result<ReturnableUser, Error> {
        use self::schema::persons::dsl::*;
        use self::schema::persons::id as user_id;

        let database = Database::init();
        let mut connection = database.connection;

        let user = persons
            .filter(user_id.eq(person))
            .select(Persons::as_select())
            .first::<Persons>(&mut connection)?;

        match PersonAddresses::belonging_to(&user)
            .select(PersonAddresses::as_select())
            .load::<PersonAddresses>(&mut connection)
        {
            Ok(res) => Ok(ReturnableUser {
                person: user,
                address: res,
            }),
            Err(err) => {
                println!(":ORDENNE:database:client:person() exception: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn all() -> Result<Vec<ReturnableUser>, Error> {
        let database = Database::init();
        let mut connection = database.connection;

        match persons::table
            .select(Persons::as_select())
            .get_results::<Persons>(&mut connection)
        {
            Ok(res) => {
                let complete_user = res
                    .into_iter()
                    .map(|client| {
                        let addresses = PersonAddresses::belonging_to(&client)
                            .select(PersonAddresses::as_select())
                            .load::<PersonAddresses>(&mut connection)
                            .unwrap();
                        ReturnableUser {
                            person: client,
                            address: addresses,
                        }
                    })
                    .collect::<Vec<ReturnableUser>>();

                Ok(complete_user)
            }
            Err(err) => {
                println!(":ORDENNE:database:client:all() exception: {:?}", err);
                Err(err)
            }
        }
    }
}
