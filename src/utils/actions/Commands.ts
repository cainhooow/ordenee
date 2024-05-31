import type { Action } from '../../types/Command/Action';
import { appWindow } from '@tauri-apps/api/window';

export const commands: Array<Action> = [
	{
		action: [
			'Voltar para página inicial',
			'Onde está a página inicial?',
			'Me leve para a página principal'
		],
		command: {
			name: 'go-to-home-page',
			type: 'redirect',
			code: 100,
			description: 'Voltar para a página inicial',
			run(...args) {
				window.location.href = '/';
			}
		}
	},
	{
		action: 'Adicionar formas de pagamentos',
		command: {
			name: 'go-to-payment-methods',
			type: 'redirect',
			code: 101,
			description: 'Ir para formas de pagamentos',
			run() {
				window.location.href = '/paymentmethods/new-method';
			}
		}
	},
	{
		action: ['Fechar o aplicativo', 'Close App'],
		command: {
			name: 'close-app',
			type: 'app-action',
			code: 0,
			description: 'Close app',
			async run() {
				await appWindow.close();
			}
		}
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
