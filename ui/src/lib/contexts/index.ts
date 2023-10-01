import type { createCountdownStore } from "$lib/stores/countdown"

export const routeContext = 'routes'
export const countdownContext = 'countdown'
export type CountdownContextType = {
    countdown: ReturnType<typeof createCountdownStore>,
    snapshotEndCountdown: ReturnType<typeof createCountdownStore>,
}