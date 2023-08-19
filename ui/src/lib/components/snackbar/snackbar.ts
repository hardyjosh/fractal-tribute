import { writable } from "svelte/store";

interface SnackbarInstance {
    message: string;
    id: number;
}
let id = 0;
export const snackBars = writable([] as SnackbarInstance[]);
export function addSnackBar(message: string) {
    id++
    snackBars.update((snackBars) => {
        const newSnackBar: SnackbarInstance = {
            message,
            id,
        };
        return [...snackBars, newSnackBar];
    });
    setTimeout(() => {
        snackBars.update((snackBars) => {
            return snackBars.filter((snackBar) => snackBar.id !== id);
        });
    }, 5000);
}