import { readFileSync } from "node:fs";

const input = JSON.parse(readFileSync("input.txt", "utf8").trim()) as unknown;

const getJsonNumbers = (data: unknown): number[] => {
        if (typeof data === 'string') {
                return [];
        }
        if (typeof data === 'number') {
                return [data]
        }

        if (typeof data !== 'object' || !data) {
                throw new Error("Malformed input");
        }

        return (data instanceof Array ? data : Object.values(data)).flatMap(getJsonNumbers);
}

const sum = getJsonNumbers(input).reduce((acc, x) => acc + x);

console.log(sum);

const getNonRedJsonNumbers = (data: unknown): number[] => {
        if (typeof data === 'string') {
                return [];
        }
        if (typeof data === 'number') {
                return [data]
        }

        if (data instanceof Array) {
                return data.flatMap(getNonRedJsonNumbers);
        }

        if (typeof data !== 'object' || !data) {
                throw new Error("Malformed input");
        }

        const values = Object.values(data);
        if (values.includes("red")) {
                return [];
        }

        return values.flatMap(getNonRedJsonNumbers);
}

const sum2 = getNonRedJsonNumbers(input).reduce((acc, x) => acc + x);

console.log(sum2);
