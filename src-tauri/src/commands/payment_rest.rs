#[tauri::command]
pub fn add_payment(payment: String) {
    println!(":ORDENNE:command:add_payment()->{:#?}", payment);
}

#[tauri::command]
pub fn load_payments() {

}