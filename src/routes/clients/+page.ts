import type { Person, ReturnablePerson } from '../../types/Person';

export async function load() {
    const { invoke } = await import('@tauri-apps/api/core');

	let clients = await invoke('load_clients')
		.then((res: any) => {
			return res.map((a: any) => (a as ReturnablePerson[]));
		})
		.catch((err) => {
			console.error(err);
		});
		
	console.log(clients);	

	return { clients };
}