use common::database::{
    imp::person_addresses::AddressBase,
    models::{PersonAddresses, Persons},
};

#[tauri::command]
pub fn create_address(address: String) -> Result<PersonAddresses, ()> {
    let address = serde_json::from_str::<AddressBase>(&address);

    let address = match address {
        Ok(address) => address,
        Err(err) => {
            println!(":ORDENEE:create_address() -> execption: {:?}", err);
            return Err(());
        }
    };

    match PersonAddresses::create(address) {
        Ok(address) => Ok(address),
        Err(err) => {
            println!("ORDENEE:create_address() -> execption: {:?}", err);
            Err(())
        }
    }
}
