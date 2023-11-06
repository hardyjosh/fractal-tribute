import { writable, type Writable } from "svelte/store";

export const language: Writable<Language> = writable("en");

export type Language = 'en' | 'tr';