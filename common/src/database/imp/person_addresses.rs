use diesel::{
    prelude::Insertable,
    query_dsl::methods::{FilterDsl, OrderDsl, SelectDsl},
    result::Error,
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::database::{models::PersonAddresses, schema, Database};

use super::persons::PersonsBase;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::personaddresses)]
pub struct AddressBase<'r> {
    pub address: &'r str,
    pub home_num: i32,
    pub street: Option<&'r str>,
    pub city: Option<&'r str>,
    pub person_id: i32,
}

impl PersonAddresses {
    pub fn create(address_base: AddressBase) -> Result<PersonAddresses, Error> {
        use self::schema::personaddresses::dsl::*;
        use self::schema::personaddresses::*;

        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(personaddresses)
            .values(vec![&address_base])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENEE:database:address:create()");

                let last_address = personaddresses
                    .select(id)
                    .order(id.desc())
                    .first::<i32>(&mut connection)?;

                let result_address = Self::address(last_address)?;

                Ok(result_address)
            }
            Err(err) => {
                println!(":ORDENEE:database:address:create() exception: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn address(address_id: i32) -> Result<PersonAddresses, Error> {
        use self::schema::personaddresses::dsl::*;
        use self::schema::personaddresses::*;

        let database = Database::init();
        let mut connection = database.connection;

        match personaddresses
            .filter(id.eq(address_id))
            .select(PersonAddresses::as_select())
            .get_result(&mut connection)
        {
            Ok(result_address) => Ok(result_address),
            Err(err) => Err(err),
        }
    }
}
