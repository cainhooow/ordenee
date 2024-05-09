export async function load({ params }) {
    const { invoke } = await import("@tauri-apps/api");
    let methodId = params.methodId;

    invoke("find_payment_method", { 
        methodId
    }).then((res) => {
        
    }).catch((err) => {
        console.error(err)
    })
}