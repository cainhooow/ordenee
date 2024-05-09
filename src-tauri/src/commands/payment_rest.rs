use common::database::{imp::payment_methods::PaymentBase, models::PaymentMethods};

#[tauri::command]
pub fn add_payment(payment: String) -> String {
    println!(":ORDENNE:command:add_payment()->{:#?}", payment);

    match PaymentMethods::create(PaymentBase { name: &payment }) {
        Ok(_) => {
            let message = String::from("payment-add-success");
            serde_json::to_string(&message).unwrap()
        }
        Err(err) => {
            println!(":ORDENNE:command:add_payment() excpetion {:?}", err);

            let message = String::from("payment-add-error");
            serde_json::to_string(&message).unwrap()
        }
    }
}

#[tauri::command]
pub fn load_payments() {}
