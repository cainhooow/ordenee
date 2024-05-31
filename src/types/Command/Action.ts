import type { Command } from "./Command"

export type Action = {
    /**
     * @property Short description for a app action 
     */
    action: string | string[],
    /**
     * @property Command to execute a action
     */
    command: Command
}