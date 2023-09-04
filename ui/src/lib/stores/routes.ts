import { writable, type Writable } from "svelte/store";
import Gallery from "../../routes/Gallery.svelte";
import Home from "../../routes/Home.svelte";

type Route = (typeof routes)[number];
type RouteName = Route["name"];

export const routes = [
    {
        name: "Play",
        component: Home,
    },
    {
        name: "Gallery",
        component: Gallery,
    },
] as const;

export const currentRoute: Writable<Route> = writable(routes[0]);

export const setRoute = (name: RouteName) => {
    const newRoute = routes.find((r) => r.name === name);
    if (newRoute)
        currentRoute.set(newRoute);
};