import { readFileSync } from "node:fs";
import { max, min } from "../lib/math.js";

const WEAPONS_INFO = `Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0`.split("\n");

const ARMOUR_INFO = `Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5`.split("\n");

const RINGS_INFO = `Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3`.split("\n");

const bossInfo = readFileSync("input.txt", "utf8").trim();

type Item = {
  cost: number;
  damage: number;
  armour: number;
};

const ITEM_REGEX = /(\d+)\s+(\d+)\s+(\d+)$/;

const itemize = (row: string) => {
  const match = ITEM_REGEX.exec(row);

  if (!match) {
    throw new Error(`Bad row ${row}`);
  }

  return {
    cost: parseInt(match[1], 10),
    damage: parseInt(match[2], 10),
    armour: parseInt(match[3], 10),
  };
};

const weapons = WEAPONS_INFO.map(itemize);
const armour = ARMOUR_INFO.map(itemize);
const rings = RINGS_INFO.map(itemize);

const NO_ITEM = { cost: 0, damage: 0, armour: 0 };

armour.push(NO_ITEM);
rings.push(NO_ITEM);
rings.push(NO_ITEM);

const BOSS_INFO_REGEX = /^Hit Points: (\d+)\nDamage: (\d+)\nArmor: (\d+)$/;

const bossInfoMatch = BOSS_INFO_REGEX.exec(bossInfo);

if (!bossInfoMatch) {
  throw new Error(`Bad input ${bossInfo}`);
}

const bossHitPoints = parseInt(bossInfoMatch[1], 10);
const bossAttack = parseInt(bossInfoMatch[2], 10);
const bossArmour = parseInt(bossInfoMatch[3], 10);

type Character = {
  damage: number;
  armour: number;
  cost: number;
  wins: boolean;
};

const PLAYER_HP = 100;

const generateCharacters = (): Character[] => {
  const result: Character[] = [];

  for (const weaponChoice of weapons) {
    for (const armourChoice of armour) {
      for (let ring1Index = 0; ring1Index < rings.length; ring1Index++) {
        for (let ring2Index = 0; ring2Index < rings.length; ring2Index++) {
          // only one of each ring
          if (ring1Index === ring2Index) {
            continue;
          }
          const ring1 = rings[ring1Index];
          const ring2 = rings[ring2Index];

          result.push({
            cost:
              weaponChoice.cost + armourChoice.cost + ring1.cost + ring2.cost,
            damage: weaponChoice.damage + ring1.damage + ring2.damage,
            armour: armourChoice.armour + ring1.armour + ring2.armour,
            wins: false,
          });
        }
      }
    }
  }

  return result;
};

const pcOptions = generateCharacters();

const battle = (pc: Character): boolean => {
  let bossHP = bossHitPoints;
  let pcHP = PLAYER_HP;

  const playerDamage = Math.max(pc.damage - bossArmour, 1);
  const bossDamage = Math.max(bossAttack - pc.armour, 1);
  for (;;) {
    bossHP -= playerDamage;
    if (bossHP <= 0) {
      return true;
    }

    pcHP -= bossDamage;
    if (pcHP <= 0) {
      return false;
    }
  }
};

pcOptions.forEach((pc) => {
  pc.wins = battle(pc);
});

const minCost = min(
  pcOptions.filter(({ wins }) => wins).map(({ cost }) => cost),
);

console.log(minCost);

const maxCost = max(
  pcOptions.filter(({ wins }) => !wins).map(({ cost }) => cost),
);

console.log(maxCost);

console.log(pcOptions.find(({ cost, wins }) => cost === maxCost && !wins));
