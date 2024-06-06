import { setContext } from "svelte";
import { writable } from "svelte/store";

export const ordeneeIADialog = writable(false);
export const ordeneeIADialogResults = writable([] as string[]);