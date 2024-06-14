import type { Payment } from '../../types/Payment';

export async function load({ params }) {
	console.log(params);
	const { invoke } = await import('@tauri-apps/api/core');

	let methods = await invoke('load_payments')
		.then((res: any) => JSON.parse(res) as Array<Payment>)
		.catch((err) => {
			console.error(err);
		});

    return {
        methods
    }    
}
