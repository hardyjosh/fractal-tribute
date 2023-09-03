import { writable } from "svelte/store";

interface ToastInstance {
    type: "success" | "error" | "info";
    message: string;
    id: number;
}
let id = 0;
export const toasts = writable([] as ToastInstance[]);
export function addToast(type: "success" | "error" | "info", message: string) {
    id++
    toasts.update((toasts) => {
        const newToast: ToastInstance = {
            type,
            message,
            id,
        };
        return [...toasts, newToast];
    });
    setTimeout(() => {
        toasts.update((toasts) => {
            return toasts.filter((toast) => toast.id !== id);
        });
    }, 5000);
}