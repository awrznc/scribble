/**
 * @param {any[]} items
 * @returns
 */
export function randomItem(items) {
    return items[Math.floor(Math.random() * items.length)]
}

Array.prototype.sample = function () {
    return randomItem(this);
};

export default Array;
