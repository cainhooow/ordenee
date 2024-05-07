use diesel::{prelude::Insertable, RunQueryDsl};
use crate::database::{models::PaymentMethods, schema::{self}, Database};

#[derive(Insertable)]
#[diesel(table_name = schema::paymentmethods)]
pub struct PaymentBase<'r> {
    pub name: &'r str,
}

impl PaymentMethods {
    pub fn create(payment: PaymentBase) {
        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(schema::paymentmethods::table)
            .values(vec![&payment])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:payment:create()");
            }
            Err(err) => {
                println!(":ORDENNE:database:payment:create() exception: {:?}", err);
            }
        }
    }

    pub fn find() {}

    pub fn all() {
        use self::schema::paymentmethods;

        let database = Database::init();
        let mut connection = database.connection;

        match paymentmethods::table.get_results::<PaymentMethods>(&mut connection)
        {
            Ok(res) => {
                println!(":ORDENNE:database:payment:all() {:?}", res);
            },
            Err(err) => {
                println!(":ORDENNE:database:payment:all() exception: {:?}", err);
            }
        }
    }
}
