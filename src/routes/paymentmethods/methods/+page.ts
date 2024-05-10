import { page } from '$app/stores';
import type { Payment } from '../../../types/Payment.js';

export async function load(req) {
    const { invoke } = await import("@tauri-apps/api");
    let methodId = req.url.searchParams.get("method");

    if (methodId == null) return;

    const method: Payment = await invoke('find_payment_method', {
        methodId: parseInt(methodId)
    })
    .then((data: any) => JSON.parse(data)).catch((err) => {
        console.error(err)
    })

    return {
        method
    }
    
}