type CommandType = 'create' | 'delete' | 'update' | 'redirect' | 'app-action';
type CommandFun = (...args: any) => Promise<void> | void;

export type Command = {
	/**
	 * @property Command name, slower command name.
	 * example:
	 * ```ts
	 * const command: Command = {
	 *     name: 'create-user'
	 *     ...
	 * }
	 * ```
	 */
	name: string;
	/**
	 * @property Command type for filter command category or command
	 * level
	 *
	 * ```ts
	 * if (command.type === 'app-action') {
	 *     // action for app.... close/open system dialog....
	 * }
	 * ```
     * 
     * ```ts
     * if (command.type === 'create') {
     *     // run item to create
     *     user.create({...})
     * }
     * ```
	 */
	type: CommandType;
	/**
	 * @property Command code for identity, filter command category or perm level
	 */
	code: number;
	/**
	 * @property Command description for describe action
	 */
	description: string;
	/**
	 * @property Command run callback
	 */
	run: CommandFun;
};
