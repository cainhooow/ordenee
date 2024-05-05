use std::{borrow::Borrow, error::Error};

use crate::fs;
use diesel::{connection, sqlite::Sqlite, Connection, SqliteConnection};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("F:\\ordenee\\common\\src\\migrations");
  
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
