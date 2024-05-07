pub mod models;
pub mod schema;
pub mod imp;

use crate::fs;
use diesel::{sqlite::Sqlite, Connection, SqliteConnection};
use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations =

embed_migrations!("F:\\ordenee\\common\\src\\database\\migrations");

pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    pub fn init() -> Self {
        println!(":ORDENNE:database:init()");

        let mut database_url = fs::AppDirStruct::get();
        database_url.push_str("/data.sqlite");

        let mut connection = SqliteConnection::establish(&database_url).unwrap();
        Self::run_migrations(&mut connection).unwrap();

        Database { connection }
    }

    pub fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        println!(":ORDENNE:database:run_migrations()");
        // This will run the necessary migrations.
        //
        // See the documentation for `MigrationHarness` for
        // all available methods.
        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
