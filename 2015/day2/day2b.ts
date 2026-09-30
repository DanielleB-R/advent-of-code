import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const PACKAGE_REGEX = /^(\d+)x(\d+)x(\d+)$/;

const packages: [number, number, number][] = input.map((row) => {
        const match = PACKAGE_REGEX.exec(row);
        if (!match) {
                throw new Error(`bad row ${row}`);
        }

        return [
                parseInt(match[1]!, 10),
                parseInt(match[2]!, 10),
                parseInt(match[3]!, 10),
        ];
});

const totalArea = packages.reduce((sum, [h, w, l]) => {
        const smallestPerimeter = 2 * (h + w + l - Math.max(h, w, l));
        const volume = h * w * l;

        return sum + smallestPerimeter + volume;
}, 0);

console.log(totalArea);
