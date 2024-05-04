use std::error::Error;

use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use crate::fs;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("F:\\ordenee\\common\\src\\migrations");

pub fn init() {
    println!(":ORDENNE:database:init()");

    let mut db_str = fs::AppDirStruct::get();
    db_str.push_str("/data.sqlite");

    let connection = &mut SqliteConnection::establish(
        &db_str
    ).unwrap();
    
    run_migrations(connection).unwrap();
}
 fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!(":ORDENNE:database:run_migrations()");
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}