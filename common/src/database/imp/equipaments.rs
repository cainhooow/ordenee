use diesel::{prelude::Insertable, RunQueryDsl};

use crate::database::{models::Equipaments, schema, Database};

#[derive(Insertable)]
#[diesel(table_name = schema::equipaments)]
pub struct EquipamentBase<'r> {
    pub name: &'r str,
    pub serie: &'r str,
    pub model: &'r str,
    pub description: &'r str,
}

impl Equipaments {
    pub fn create(equipament: EquipamentBase) {
        let database = Database::init();
        let mut connection = database.connection;

        match diesel::insert_into(schema::equipaments::table)
            .values(vec![&equipament])
            .execute(&mut connection)
        {
            Ok(_) => {
                println!(":ORDENNE:database:equipament:create()");
            }
            Err(err) => {
                println!(":ORDENNE:database:equipament:create() exception: {:?}", err);
            }
        }
    }
    pub fn find() {}
    pub fn all() {
        use self::schema::equipaments;

        let database = Database::init();
        let mut connection = database.connection;

        match equipaments::table.get_results::<Equipaments>(&mut connection) {
            Ok(res) => {
                println!(":ORDENNE:database:equipaments:all() {:?}", res);
            }
            Err(err) => {
                println!(":ORDENNE:database:equipaments:all() exception: {:?}", err);
            }
        }
    }
}
