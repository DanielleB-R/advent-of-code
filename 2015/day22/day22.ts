import { readFileSync } from "node:fs";

const bossInfo = readFileSync("input.txt", "utf8").trim();

const BOSS_INFO_REGEX = /^Hit Points: (\d+)\nDamage: (\d+)$/;

const bossInfoMatch = BOSS_INFO_REGEX.exec(bossInfo);

if (!bossInfoMatch) {
        throw new Error(`Bad input ${bossInfo}`);
}

const bossMaxHP = parseInt(bossInfoMatch[1], 10);
const bossDamage = parseInt(bossInfoMatch[2], 10);

type BattleState = {
        bossHP: number;
        playerHP: number;
        currentMana: number;
        manaSpent: number;
        shieldTurnsLeft: number;
        poisonTurnsLeft: number;
        rechargeTurnsLeft: number;
};

const initialState: BattleState = {
        bossHP: bossMaxHP,
        playerHP: 50,
        currentMana: 500,
        manaSpent: 0,
        shieldTurnsLeft: 0,
        poisonTurnsLeft: 0,
        rechargeTurnsLeft: 0,
}

type Action = 'Magic Missile' | 'Drain' | 'Shield' | 'Poison' | 'Recharge';

const poisonDamage = (state: BattleState): number => (state.poisonTurnsLeft > 0 ? 3 : 0);

const rechargeMana = (state: BattleState): number => (state.rechargeTurnsLeft > 0 ? 101 : 0);

const armour = (state: BattleState): number => (state.shieldTurnsLeft > 0 ? 7 : 0);

const startTurn = (state: BattleState): BattleState => ({
        ...state,
        bossHP: state.bossHP - poisonDamage(state),
        currentMana: state.currentMana + rechargeMana(state),
        shieldTurnsLeft: Math.max(state.shieldTurnsLeft - 1, 0),
        poisonTurnsLeft: Math.max(state.poisonTurnsLeft - 1, 0),
        rechargeTurnsLeft: Math.max(state.rechargeTurnsLeft - 1, 0),
})

const MAGIC_MISSILE_COST = 53;
const DRAIN_COST = 73;
const SHIELD_COST = 113;
const POISON_COST = 173;
const RECHARGE_COST = 229

const ACTIONS: Record<Action, (state: BattleState) => BattleState | null> = {
        'Magic Missile': (state) => ({
                ...state,
                currentMana: state.currentMana - MAGIC_MISSILE_COST,
                manaSpent: state.manaSpent + MAGIC_MISSILE_COST,
                bossHP: state.bossHP - 4,
        }),
        'Drain': (state) => ({
                ...state,
                currentMana: state.currentMana - DRAIN_COST,
                manaSpent: state.manaSpent + DRAIN_COST,
                bossHP: state.bossHP - 2,
                playerHP: state.playerHP + 2,
        }),
        'Shield': (state) => (state.shieldTurnsLeft > 0 ? null : {
                ...state,
                currentMana: state.currentMana - SHIELD_COST,
                manaSpent: state.manaSpent + SHIELD_COST,
                shieldTurnsLeft: 6,
        }),
        'Poison': (state) => (state.poisonTurnsLeft > 0 ? null : {
                ...state,
                currentMana: state.currentMana - POISON_COST,
                manaSpent: state.manaSpent + POISON_COST,
                poisonTurnsLeft: 6,
        }),
        'Recharge': (state) => (state.rechargeTurnsLeft > 0 ? null : {
                ...state,
                currentMana: state.currentMana - RECHARGE_COST,
                manaSpent: state.manaSpent + RECHARGE_COST,
                rechargeTurnsLeft: 5,
        })
}

const actionList = Object.keys(ACTIONS) as Action[];

const bossAction = (state: BattleState): BattleState => ({
        ...state,
        playerHP: state.playerHP - bossDamage + armour(state)
})

type AdditionalAction = (state: BattleState) => BattleState;

const turn = (state: BattleState, action: Action, additionalAction: AdditionalAction): BattleState | null => {
        const afterPlayerInit = additionalAction(startTurn(state));

        if (afterPlayerInit.playerHP <= 0) {
                return null;
        }

        const afterPlayerState = ACTIONS[action](afterPlayerInit);

        if (!afterPlayerState || afterPlayerState.currentMana < 0) {
                return null;
        }

        if (afterPlayerState.bossHP <= 0) {
                return afterPlayerState;
        }

        return bossAction(startTurn(afterPlayerState));
}

const resolve = (additionalAction: AdditionalAction = (x) => x): number => {
        let minimum = Infinity;

        const minimalManaVictory = (state: BattleState): boolean => {
                if (state.bossHP <= 0) {
                        minimum = Math.min(minimum, state.manaSpent);
                        return true;
                }

                if (state.playerHP <= 0) {
                        return false;
                }

                if (state.manaSpent >= minimum) {
                        return true;
                }


                const nextSteps = actionList.map((action) => turn(state, action, additionalAction)).filter((next) => next !== null);

                return nextSteps.map(minimalManaVictory).some((result) => result);
        }

        minimalManaVictory(initialState);
        return minimum;
}


console.log(resolve());

console.log(resolve((state) => ({ ...state, playerHP: state.playerHP - 1 })))
