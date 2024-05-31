use common::database::{imp::persons::PersonsBase, models::Persons};

#[tauri::command]
pub fn add_client(client: String) -> Result<String, ()> {
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
        Ok(_) => Ok(String::from("created")),
        Err(err) => {
            println!(":ORDENNE:command:add_client() excpetion {:?}", err);
            Err(())
        }
    }
}

#[tauri::command]
pub fn load_clients() -> Result<String, ()> {
    println!(":ORDENNE:command:load_clients()");

    match Persons::all() {
        Ok(persons) => {
            let message = serde_json::to_string(&persons).unwrap();
            Ok(message)
        }
        Err(err) => {
            println!(":ORDENNE:command:load_clients excpetion {:?}", err);
            Err(())
        }
    }
}
