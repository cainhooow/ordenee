import type { Action } from '../../types/Command/Action';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api';
import { OrdeneeNer } from '../ordenee/OrdeneeNer';
import { ordeneeIADialog, ordeneeIADialogResults } from '../../store';

export const commands: Array<Action> = [
	{
		action: [
			'Voltar para página inicial',
			'Onde está a página inicial?',
			'Me leve para a página principal'
		],
		command: {
			name: '/goto-home',
			type: 'redirect',
			code: 100,
			description: 'Voltar para a página inicial',
			run(...args) {
				ordeneeIADialogResults.update((vs) => [...vs, "Certo! Indo para a página inicial..."]);
				setTimeout(() => {
					window.location.href = '/';
				}, 3000)
			}
		},
		filtable: true
	},
	{
		action: ['Adicionar um novo cliente', 'Adicionar cliente', 'Página para adicionar clientes'],
		command: {
			name: '/goto-clientadd',
			type: 'redirect',
			code: 102,
			description: 'Ir para clientes',
			run(...args) {
				ordeneeIADialogResults.update((vs) => [...vs, "Certo! Vamos para a página de adicionar clientes..."]);

				setTimeout(() => {
					window.location.href = '/clients/new-client';
				}, 3000)
			}
		},
		filtable: true
	},

	{
		action: [
			'Adicionar um o cliente com o nome',
			'Criar um cliente com o nome',
			'Crie um cliente com o nome'
		],
		command: {
			name: '/goto-clientadd',
			type: 'redirect',
			code: 103,
			description: 'Ir para clientes',
			run(...args) {
				const text: string = args[0].searchText;
				ordeneeIADialogResults.update((vs) => [...vs, `Ok, Certo! aguarde...`]);

				new OrdeneeNer().result(text).then(async (result) => {
					ordeneeIADialogResults.update((vs) => [...vs, `Procurando o nome do cliente...`]);

					const splitedText = text.split(' ');
					ordeneeIADialogResults.update((vs) => [...vs, `Estou quebrando o seu texto em pedaços...`]);

					const persons = result.filter((entities) => {
						if (entities.label.includes('PERSON')) {
							return splitedText.includes(entities.token);
						}
					});
					ordeneeIADialogResults.update((vs) => [...vs, `Certo! Agora vamos adicionar o cliente...`]);

					const mountName = persons.map((user) => {
						return user.token;
					});

					await invoke('add_client', {
						client: JSON.stringify({
							name: mountName.join(' '),
							email: null,
							person_id: null,
							tel_num: null,
							is_technical: false
						})
					}).then((res) => {
						ordeneeIADialogResults.update((vs) => [...vs, `Pronto! O cliente ${mountName.join(' ')} foi adicionado com sucesso! Vou lhe redirecionar para a página de clientes.`]);

						setTimeout(() => {
							window.location.href = '/clients';
						}, 4000)
					});
				});
			}
		},
		filtable: false
	},

	{
		action: [
			'Ver clientes',
			'Ver todos os clientes',
			'Página para ver os clientes',
			'Ir para clientes'
		],
		command: {
			name: '/goto-clientlist',
			type: 'redirect',
			code: 104,
			description: 'Ir para clientes',
			run(...args) {
				window.location.href = '/clients';
			}
		},
		filtable: false
	},
	{
		action: 'Adicionar formas de pagamentos',
		command: {
			name: '/goto-addpayment',
			type: 'redirect',
			code: 105,
			description: 'Ir para formas de pagamentos',
			run() {
				window.location.href = '/paymentmethods/new-method';
			}
		},
		filtable: true
	},
	{
		action: ['Formas de pagamento', 'Ir para formas de pagamento', 'Página de formas de pagamento'],
		command: {
			name: '/goto-paymentlist',
			type: 'redirect',
			code: 106,
			description: 'Ir para formas de pagamentos',
			run() {
				window.location.href = '/paymentmethods';
			}
		},
		filtable: false
	},
	{
		action: ['Fechar o aplicativo', 'Close App'],
		command: {
			name: '/close-app',
			type: 'app-action',
			code: 0,
			description: 'Close app',
			async run() {
				await appWindow.close();
			}
		},
		filtable: false
	}
];

export const mapCommandsAction = () => {
	return commands.map((act) => act.action).flat();
};

export const mapCommandsName = () => {
	return commands.map((act) => act.command.name);
};

export const getCommand = (cmdName: string) => {
	return commands.find((_action) => {
		if (cmdName === _action.command.name) {
			return _action;
		}

		if (typeof _action.action === 'object') {
			if (_action.action.includes(cmdName)) {
				return _action;
			}
		}

		if (cmdName === _action.action) {
			return _action;
		}
	});
	// return commands.find((_action) => cmdName === _action.command.name || cmdName === _action.action);
};

export const handleCommand = async (cmd: string, ...args: any[]) => {
	const command = getCommand(cmd);

	console.log(command);

	if (command) {
		await command.command.run(args);
	}
};
