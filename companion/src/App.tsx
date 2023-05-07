import { Component, For, JSX, Setter, Show, createSignal } from 'solid-js';

import Armament from './Armament'
import { armaments, Arm } from './Armament';
import Armor, { Armors, armors } from './Armor'
import WSelector, { SPair, Weights, defaultMins, defaultWeights, weights_map } from './WSelector';
import compute_best, { ALst, selection_total } from './Compute';

const [w1, setW1] = createSignal("Empty")
const [w2, setW2] = createSignal("Empty")
const [w3, setW3] = createSignal("Empty")
const [w4, setW4] = createSignal("Empty")
const [w5, setW5] = createSignal("Empty")
const [w6, setW6] = createSignal("Empty")

const SWeaponSelector = (nm: string, aarray: Arm[], setS: Setter<string>) => {
  return <div class="col">
    <label for={"s_" + nm} class="form-label">{nm}</label>
    <select class="form-select" id={"s_" + nm} onChange={(op) => setS(op.currentTarget.value)}>
      <option value="empty">Empty</option>
      <For each={aarray.filter((r) => r.owned())}>{(armr) => {
        return <option value={armr.name}>
          {armr.name}
        </option>
      }}
      </For>
    </select>
  </div>
}

const weapons_weight = () => {
  let held = [w1(), w2(), w3(), w4(), w5(), w6()].filter((s) => s !== "Empty")
  let flt = armaments.filter((a) => held.includes(a.name)).map((a) => a.weight)
  return flt.reduce((a, b) => a + b, 0)
}

const weight_budget = (mode: string, total: number, used: number) => {
  let max = total;
  switch (mode) {
    case "30%":
      max = total * 0.3
      break;
    case "70%":
      max = total * 0.7
      break;
  }
  let rm = max - used;
  if (rm > 0) {
    return rm
  } else {
    return 0
  }
}

const WeaponSelector = (weapons: Arm[]) => {
  return <div>
    <div class="row">
      {SWeaponSelector("Weapon 1", weapons, setW1)}
      {SWeaponSelector("Weapon 2", weapons, setW2)}
      {SWeaponSelector("Weapon 3", weapons, setW3)}
    </div>
    <div class="row">
      {SWeaponSelector("Weapon 4", weapons, setW4)}
      {SWeaponSelector("Weapon 5", weapons, setW5)}
      {SWeaponSelector("Weapon 6", weapons, setW6)}
    </div>
  </div >
}

function best_col(r: ALst | null): JSX.Element {
  if (r === null) {
    return <tr>
      <td> &hyphen; </td>
    </tr>
  } else {
    return <tr>
      <td> {r.n} </td>
      <td> {Number(r.weight).toFixed(1)} </td>
      <td> {Number(r.values.resistances.poise).toFixed(0)} </td>
      <td> {Number(r.values.absorptions.fire).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.holy).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.lightning).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.magic).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.physical).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.pierce).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.slash).toFixed(1)} </td>
      <td> {Number(r.values.absorptions.strike).toFixed(1)} </td>
      <td> {Number(r.values.resistances.focus).toFixed(0)} </td>
      <td> {Number(r.values.resistances.immunity).toFixed(0)} </td>
      <td> {Number(r.values.resistances.robustness).toFixed(0)} </td>
      <td> {Number(r.values.resistances.vitality).toFixed(0)} </td>
    </tr>
  }

}

function best_calc(budget: number, weights: Weights<SPair<number>>, mins: Weights<SPair<number>>, armors: Armors): JSX.Element {
  const nweights: Weights<number> = weights_map(weights, (x) => x.access())
  const nmin: Weights<number> = weights_map(mins, (x) => x.access())
  let r = compute_best(budget, nweights, nmin, armors)

  return <table class="table">
    <thead>
      <tr>
        <th scope="col">Name</th>
        <th scope="col">Weight</th>
        <th scope="col">Poise</th>
        <th scope="col">Fire</th>
        <th scope="col">Holy</th>
        <th scope="col">Lig.</th>
        <th scope="col">Mag.</th>
        <th scope="col">Phy.</th>
        <th scope="col">Pie.</th>
        <th scope="col">Sla.</th>
        <th scope="col">Str.</th>
        <th scope="col">Foc.</th>
        <th scope="col">Imm.</th>
        <th scope="col">Rob.</th>
        <th scope="col">Vit.</th>
      </tr>
    </thead>
    <tbody>
      {best_col(r.head)}
      {best_col(r.body)}
      {best_col(r.arms)}
      {best_col(r.legs)}
      {best_col(selection_total(r))}
    </tbody>
  </table>
}

function App() {
  const [filter, setFilter] = createSignal("")
  const [weightmode, setWeightmode] = createSignal("70%")
  const [totalWeight, setTotalWeight] = createSignal(45)
  const [extraWeight, setExtraWeight] = createSignal(0)
  const weights = defaultWeights();
  const mins = defaultMins();
  return (
    <div>
      <div class="row">
        <div class="col">
          <h3>Multipliers</h3>
          {WSelector("weights", weights)}
        </div>
        <div class="col">
          <h3>Minimal values</h3>
          {WSelector("mins", mins)}
        </div>
      </div>

      <div class="row">
        {best_calc(weight_budget(weightmode(), totalWeight(), weapons_weight() + extraWeight()), weights, mins, armors)}
      </div>

      <div class="row">
        <div class="col">
          <label for="total_weight" class="form-label">Carrying capacity</label>
          <input class="form-control" id="total_weight" type="number" aria-describedby="total_weight_help" value={totalWeight()}
            onInput={(e) => setTotalWeight(parseInt(e.currentTarget.value))} />
          <div id="total_weight_help" class="form-text">Total weight you can bear</div>
        </div>
        <div class="col">
          <label for="extra_weight" class="form-label">Extra weight</label>
          <input class="form-control" id="extra_weight" type="number" aria-describedby="extra_weight_help" value={extraWeight()}
            onInput={(e) => setExtraWeight(parseInt(e.currentTarget.value))} />
          <div id="extra_weight_help" class="form-text">extra weight you can bear</div>
        </div>
        <div class="col">
          <label for="wpncweight" class="form-label">Weapons + charms weight</label>
          <input class="form-control" id="wpncweight" disabled value={weapons_weight() + extraWeight()} />
        </div>
        <div class="col">
          <label for="wmode" class="form-label">Mode</label>
          <select class="form-select" id="select_wmode" onChange={(op) => setWeightmode(op.currentTarget.value)}>
            <option value="30%">Less than 30%</option>
            <option value="70%" selected>Less than 70%</option>
            <option value="100%">Less than 100%</option>
          </select>
        </div>
        <div class="col">
          <label for="wbudget" class="form-label">Weight budget</label>
          <input class="form-control" id="wbudget" disabled value={Number(weight_budget(weightmode(), totalWeight(), weapons_weight())).toFixed(1)} />
        </div>
      </div>
      {WeaponSelector(armaments)}
      <div class="row">
        <div class="col">
          <label for="filter_input" class="form-label">Filter</label>
          <input class="form-control" id="filter_input" aria-aria-describedby="filter_help" onInput={(e) => setFilter(e.currentTarget.value)}></input>
          <div id="filter_help" class="form-text">Type here to quickly select elements</div>
        </div>
      </div>
      <div class="row">
        {Armor(filter())}
      </div >
      <div class="row">
        {Armament(filter())}
      </div>
    </div>
  );
};

export default App;