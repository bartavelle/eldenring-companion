import { Accessor, Component, JSX, Setter, createSignal } from "solid-js";

type Weights<T> = {
  absorptions: {
    fire: T,
    holy: T,
    lightning: T,
    magic: T,
    physical: T,
    pierce: T,
    slash: T,
    strike: T
  },
  resistances: {
    focus: T,
    immunity: T,
    poise: T,
    robustness: T,
    vitality: T
  },
}

type FAB<A, B> = (a: A) => B;

function weights_map<A, B>(i: Weights<A>, f: FAB<A, B>): Weights<B> {
  return {
    absorptions: {
      fire: f(i.absorptions.fire),
      holy: f(i.absorptions.holy),
      lightning: f(i.absorptions.lightning),
      magic: f(i.absorptions.magic),
      physical: f(i.absorptions.physical),
      pierce: f(i.absorptions.pierce),
      slash: f(i.absorptions.slash),
      strike: f(i.absorptions.strike)
    },
    resistances: {
      focus: f(i.resistances.focus),
      immunity: f(i.resistances.immunity),
      poise: f(i.resistances.poise),
      robustness: f(i.resistances.robustness),
      vitality: f(i.resistances.vitality)
    }
  }
}

type genFn<T> = (name: string) => T;

function genWeights<T>(f: genFn<T>): Weights<T> {
  return {
    absorptions: {
      fire: f("fire"),
      holy: f("holy"),
      lightning: f("lightning"),
      magic: f("magic"),
      physical: f("physical"),
      pierce: f("pierce"),
      slash: f("slash"),
      strike: f("strike"),
    },
    resistances: {
      focus: f("focus"),
      immunity: f("immunity"),
      poise: f("poise"),
      robustness: f("robustness"),
      vitality: f("vitality"),
    }
  }
}

class SPair<T> {
  access: Accessor<T>;
  set: Setter<T>;

  constructor(t: T) {
    const [access, set] = createSignal(t);
    this.access = access;
    this.set = set;
  }
}

function defaultWeights(): Weights<SPair<number>> {
  return genWeights((name) => {
    switch (name) {
      case "physical":
      case "pierce":
      case "slash":
      case "strike":
        return new SPair(1)
      default:
        return new SPair(0)
    }
  })
}

function defaultMins(): Weights<SPair<number>> {
  return genWeights((name) => new SPair(0))
}

function eselector(i: string, n: string, s: SPair<number>): JSX.Element {
  let id = i + "_" + n + "_id";
  return <div class="row">
    <label for={id} class="form-label col">{n}</label>
    <input type="number" class="form-control col" id={id} value={s.access()} onChange={(v) => s.set(parseFloat(v.currentTarget.value))} />
  </div>
}

function WSelector(i: string, c: Weights<SPair<number>>): JSX.Element {
  return <div class="row">
    {eselector(i, "fire", c.absorptions.fire)}
    {eselector(i, "holy", c.absorptions.holy)}
    {eselector(i, "lightning", c.absorptions.lightning)}
    {eselector(i, "magic", c.absorptions.magic)}
    {eselector(i, "physical", c.absorptions.physical)}
    {eselector(i, "pierce", c.absorptions.pierce)}
    {eselector(i, "slash", c.absorptions.slash)}
    {eselector(i, "strike", c.absorptions.strike)}
    {eselector(i, "focus", c.resistances.focus)}
    {eselector(i, "immunity", c.resistances.immunity)}
    {eselector(i, "poise", c.resistances.poise)}
    {eselector(i, "robustness", c.resistances.robustness)}
    {eselector(i, "vitality", c.resistances.vitality)}
  </div>
};

export default WSelector
export { type Weights, type SPair, defaultWeights, defaultMins, weights_map }