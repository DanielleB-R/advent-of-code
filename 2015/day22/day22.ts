import { readFileSync } from "node:fs";
import params from "./params.json" with { type: "json" };

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
  playerHP: params.playerHP,
  currentMana: params.playerMana,
  manaSpent: 0,
  shieldTurnsLeft: 0,
  poisonTurnsLeft: 0,
  rechargeTurnsLeft: 0,
};

type Action = "Magic Missile" | "Drain" | "Shield" | "Poison" | "Recharge";

const poisonDamage = (state: BattleState): number =>
  state.poisonTurnsLeft > 0 ? params.poison.damage : 0;

const rechargeMana = (state: BattleState): number =>
  state.rechargeTurnsLeft > 0 ? params.recharge.mana : 0;

const armour = (state: BattleState): number =>
  state.shieldTurnsLeft > 0 ? params.shield.armour : 0;

const startTurn = (state: BattleState): BattleState => ({
  ...state,
  bossHP: state.bossHP - poisonDamage(state),
  currentMana: state.currentMana + rechargeMana(state),
  shieldTurnsLeft: Math.max(state.shieldTurnsLeft - 1, 0),
  poisonTurnsLeft: Math.max(state.poisonTurnsLeft - 1, 0),
  rechargeTurnsLeft: Math.max(state.rechargeTurnsLeft - 1, 0),
});

const spendMana = (state: BattleState, amount: number): BattleState => ({
  ...state,
  currentMana: state.currentMana - amount,
  manaSpent: state.manaSpent + amount,
});

const ACTIONS: Record<Action, (state: BattleState) => BattleState | null> = {
  "Magic Missile": (state) =>
    spendMana(
      {
        ...state,
        bossHP: state.bossHP - params.missile.damage,
      },
      params.missile.cost,
    ),
  Drain: (state) =>
    spendMana(
      {
        ...state,
        bossHP: state.bossHP - params.drain.amount,
        playerHP: state.playerHP + params.drain.amount,
      },
      params.drain.cost,
    ),
  Shield: (state) =>
    state.shieldTurnsLeft > 0
      ? null
      : spendMana(
          {
            ...state,
            shieldTurnsLeft: params.shield.turns,
          },
          params.shield.cost,
        ),
  Poison: (state) =>
    state.poisonTurnsLeft > 0
      ? null
      : spendMana(
          {
            ...state,
            poisonTurnsLeft: params.poison.turns,
          },
          params.poison.cost,
        ),
  Recharge: (state) =>
    state.rechargeTurnsLeft > 0
      ? null
      : spendMana(
          {
            ...state,
            rechargeTurnsLeft: params.recharge.turns,
          },
          params.recharge.cost,
        ),
};

const actionList = Object.keys(ACTIONS) as Action[];

const bossAction = (state: BattleState): BattleState => ({
  ...state,
  playerHP: state.playerHP - bossDamage + armour(state),
});

type AdditionalAction = (state: BattleState) => BattleState;

const turn = (
  state: BattleState,
  action: Action,
  additionalAction: AdditionalAction,
): BattleState | null => {
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
};

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

    const nextSteps = actionList
      .map((action) => turn(state, action, additionalAction))
      .filter((next) => next !== null);

    return nextSteps.map(minimalManaVictory).some((result) => result);
  };

  minimalManaVictory(initialState);
  return minimum;
};

console.log(resolve());

console.log(resolve((state) => ({ ...state, playerHP: state.playerHP - 1 })));
