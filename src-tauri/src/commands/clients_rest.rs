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
