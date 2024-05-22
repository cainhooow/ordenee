type CommandType = 'create' | 'delete' | 'update' | 'redirect' | 'app-action';
type CommandFun = (...args: any) => Promise<void> | void;

export type Command = {
    name: string,
    type: CommandType,
    code: number,
    description: string,
    run: CommandFun
}

