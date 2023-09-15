// countdownStore.ts
import { derived, writable } from 'svelte/store';

export type CountdownObject = {
    days: [number, number],
    hours: [number, number],
    minutes: [number, number],
    seconds: [number, number]
};

const calculateTimeRemaining = (endTimestamp: number, currentTimestamp: number): CountdownObject => {
    let diff = endTimestamp - currentTimestamp;

    if (diff < 0) {
        diff = 0;
    }

    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    const hours = Math.floor((diff / (1000 * 60 * 60)) % 24);
    const minutes = Math.floor((diff / (1000 * 60)) % 60);
    const seconds = Math.floor((diff / 1000) % 60);

    return {
        days: [Math.floor(days / 10), days % 10],
        hours: [Math.floor(hours / 10), hours % 10],
        minutes: [Math.floor(minutes / 10), minutes % 10],
        seconds: [Math.floor(seconds / 10), seconds % 10]
    };
};
export const createCountdownStore = (endTime: Date) => {
    const time = writable(Date.now(), (set) => {
        const intervalId = setInterval(() => {
            set(Date.now());
        }, 1000);
        return () => {
            clearInterval(intervalId)
        }
    });

    const countdown = derived(time, ($time) =>
        calculateTimeRemaining(endTime.valueOf(), $time)
    );
    return countdown;
};

export const formatCountdown = (countdown: CountdownObject): string => {
    const days = `${countdown.days[0]}${countdown.days[1]}`.padStart(2, '0');
    const hours = `${countdown.hours[0]}${countdown.hours[1]}`.padStart(2, '0');
    const minutes = `${countdown.minutes[0]}${countdown.minutes[1]}`.padStart(2, '0');
    const seconds = `${countdown.seconds[0]}${countdown.seconds[1]}`.padStart(2, '0');

    return `${days}d : ${hours}h : ${minutes}m : ${seconds}s`;
};