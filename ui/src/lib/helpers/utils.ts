export function formatAddress(address: string) {
    const formatted = address.slice(0, 6) + '...' + address.slice(address.length - 4, address.length);
    return formatted;
}