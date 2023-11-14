import { derived, get, writable, type Writable } from "svelte/store";
import Leaderboard from "../../routes/Leaderboard.svelte";
import Home from "../../routes/Home.svelte";
import { language } from "$lib/stores";

type Route = (typeof routes)[number];
type RouteName = Route["name"];

// export const routes = [
//     {
//         name: "Play",
//         component: Home,
//     },
//     {
//         name: "Leaderboard",
//         component: Leaderboard,
//     },
// ] as const;

export const routes = derived(language, ($language) => {
    if ($language == "en") {
        return [{
            name: "Play",
            component: Home,
            id: 0
        },
        {
            name: "Leaderboard",
            component: Leaderboard,
            id: 1
        }]
    }
    else {
        return [{
            name: "Oyna",
            component: Home,
            id: 0
        },
        {
            name: "Liderlik Panosu",
            component: Leaderboard,
            id: 1
        }]
    }
})

export const currentRoute: Writable<Route> = writable(get(routes)[0]);

export const setRoute = (name: RouteName) => {
    const newRoute = get(routes).find((r) => r.name === name);
    if (newRoute)
        currentRoute.set(newRoute);
};