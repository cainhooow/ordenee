import type { Action } from '../../types/Command/Action';
import { appWindow } from '@tauri-apps/api/window';

export const commands: Array<Action> = [
	{
		action: 'Criar formas de pagamentos comuns',
		command: {
			name: 'create-common-payment',
			type: 'create',
			code: 100,
			description: 'Criar formas de pagamento comumente usadas',
			run: () => {}
		}
	},
	{
		action: 'Adicionar formas de pagamentos',
		command: {
			name: 'goto-payment-methods',
			type: 'redirect',
			code: 101,
			description: 'Ir para formas de pagamentos',
			run: () => {}
		}
	},
	{
		action: 'Fechar o aplicativo',
		command: {
			name: 'close-app',
			type: 'app-action',
			code: 0,
			description: 'Close app',
			run: async () => {
				await appWindow.close();
			}
		}
	}
];

export const getCommand = (cmdName: string) => {
	return commands.find((_action) => cmdName === _action.command.name);
};

export const handleCommand = async (cmdName: string, ...args: any[]) => {
	const command = getCommand(cmdName);

	console.log(command);
	
	if (command) {
		await command.command.run(args);
	}
};
