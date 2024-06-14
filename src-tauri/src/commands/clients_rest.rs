use common::database::{imp::persons::{PersonsBase, ReturnableUser}, models::{PersonAddresses, Persons}};

#[tauri::command]
pub fn add_client(client: String) -> Result<Persons, ()> {
    println!(":ORDENNE:command:add_client()->{:#?}", client);

    let client = serde_json::from_str::<PersonsBase>(&client);

    let client = match client {
        Ok(cli) => cli,
        Err(err) => {
            println!(":ORDENNE:command:add_client() excpetion {:?}", err);
            return Err(());
        }
    };

    match Persons::create(client) {
        Ok(person) => Ok(person),
        Err(err) => {
            println!(":ORDENNE:command:add_client() excpetion {:?}", err);
            Err(())
        }
    }
}

#[tauri::command]
pub fn load_clients() -> Result<Vec<ReturnableUser>, ()> {
    println!(":ORDENNE:command:load_clients()");

    match Persons::all() {
        Ok(persons) => {
            Ok(persons)
        }
        Err(err) => {
            println!(":ORDENNE:command:load_clients excpetion {:?}", err);
            Err(())
        }
    }
}
