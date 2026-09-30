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
        const side1 = h * w;
        const side2 = w * l;
        const side3 = l * h;

        const smallest = Math.min(side1, side2, side3);

        return sum + 2 * side1 + 2 * side2 + 2 * side3 + smallest;
}, 0);

console.log(totalArea);
