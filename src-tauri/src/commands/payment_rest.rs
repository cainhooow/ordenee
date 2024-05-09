use common::database::{imp::payment_methods::PaymentBase, models::PaymentMethods};
#[tauri::command]
pub fn add_payment(payment: String) -> Result<String, ()> {
    println!(":ORDENNE:command:add_payment()->{:#?}", payment);

    match PaymentMethods::create(PaymentBase { name: &payment }) {
        Ok(_) => {
            let message = String::from("payment-add-success");
            Ok(serde_json::to_string(&message).unwrap())
        }
        Err(err) => {
            println!(":ORDENNE:command:add_payment() excpetion {:?}", err);
            Err(())
        }
    }
}

#[tauri::command]
pub fn find_payment_method(method_id: i32) -> Result<String, ()> {
    println!(":ORDENNE:command:find_method()->{:#?}", method_id);

    match PaymentMethods::find_by_id(method_id) {
        Ok(method) => {
            let message = serde_json::to_string(&method).unwrap();
            Ok(message)
        }
        Err(err) => {
            println!(":ORDENNE:command:find_method() excpetion {:?}", err);
            Err(())
        }
    }
}

#[tauri::command]
pub fn load_payments() -> Result<String, ()> {
    println!(":ORDENNE:command:load_payments()");

    match PaymentMethods::all() {
        Ok(methods) => {
            let message = serde_json::to_string(&methods).unwrap();
            Ok(message)
        }
        Err(err) => {
            println!(":ORDENNE:command:load_payments excpetion {:?}", err);
            Err(())
        }
    }
}
