import type { Person } from '../../types/Person';

export async function load() {
    const { invoke } = await import('@tauri-apps/api');

	let clients = await invoke('load_clients')
		.then((res: any) => JSON.parse(res) as Array<Person>)
		.catch((err) => {
			console.error(err);
		});

	return { clients };
}