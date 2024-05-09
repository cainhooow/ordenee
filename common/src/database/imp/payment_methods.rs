use crate::database::{
    models::PaymentMethods,
    schema::{self},
    Database,
};
use diesel::{
    prelude::Insertable, result::Error, ExpressionMethods, QueryDsl,
    RunQueryDsl, SelectableHelper
};
#[derive(Insertable)]
#[diesel(table_name = schema::paymentmethods)]
pub struct PaymentBase<'r> {
    pub name: &'r str,
}

impl PaymentMethods {
    pub fn create(payment: PaymentBase) -> Result<(), Error> {
        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(schema::paymentmethods::table)
            .values(vec![&payment])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:payment:create()");
                Ok(())
            }
            Err(err) => {
                println!(":ORDENNE:database:payment:create() exception: {:?}", err);
                Err(err)
            }
        }
    }

    pub fn find_by_id(method_id: i32) -> Result<PaymentMethods, Error> {
        use self::schema::paymentmethods::dsl::*;
        use self::schema::paymentmethods::*;

        let database = Database::init();
        let mut connection = database.connection;

        match paymentmethods
            .filter(id.eq(method_id))
            .select(PaymentMethods::as_select())
            .get_result(&mut connection)
        {
            Ok(payment) => {
                Ok(payment)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn all() -> Result<Vec<PaymentMethods>, Error> {
        use self::schema::paymentmethods;

        let database = Database::init();
        let mut connection = database.connection;

        match paymentmethods::table.get_results::<PaymentMethods>(&mut connection) {
            Ok(res) => {
                println!(":ORDENNE:database:payment:all() {:?}", res);
                Ok(res)
            }
            Err(err) => {
                println!(":ORDENNE:database:payment:all() exception: {:?}", err);
                Err(err)
            }
        }
    }
}
