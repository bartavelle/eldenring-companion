import { Accessor, Component, For, JSX, Setter, Show, createSignal, onMount } from 'solid-js';

import Armament from './Armament'
import { armaments, Arm } from './Armament';
import Armor, { Armors, armors } from './Armor'
import WSelector, { SPair, Weights, defaultMins, defaultWeights, weights_map } from './WSelector';
import compute_best, { ALst, selection_total } from './Compute';
import { ArmorSlots } from './Common'

const [w1, setW1] = createSignal("Empty")
const [w2, setW2] = createSignal("Empty")
const [w3, setW3] = createSignal("Empty")
const [w4, setW4] = createSignal("Empty")
const [w5, setW5] = createSignal("Empty")
const [w6, setW6] = createSignal("Empty")

const SWeaponSelector = (nm: string, aarray: Arm[], setS: Setter<string>, selected: string) => {
  return <div class="col">
    <select class="form-select" id={"s_" + nm} onChange={(op) => {
      const wname = op.currentTarget.value
      localStorage.setItem(nm, wname)
      setS(wname)
    }}>
      <option value="empty">Empty</option>
      <For each={aarray.filter((r) => r.owned())}>{(armr) => {
        return <option value={armr.name} selected={armr.name === selected}>
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
  onMount(() => {
    const w1 = localStorage.getItem("Weapon 1")
    if (w1 !== null) {
      setW1(w1)
    }
    const w2 = localStorage.getItem("Weapon 2")
    if (w2 !== null) {
      setW2(w2)
    }
    const w3 = localStorage.getItem("Weapon 3")
    if (w3 !== null) {
      setW3(w3)
    }
    const w4 = localStorage.getItem("Weapon 4")
    if (w4 !== null) {
      setW4(w4)
    }
    const w5 = localStorage.getItem("Weapon 5")
    if (w5 !== null) {
      setW5(w5)
    }
    const w6 = localStorage.getItem("Weapon 6")
    if (w6 !== null) {
      setW6(w6)
    }
  })
  return <div>
    <h4>Equipped weapons</h4>
    <div class="row">
      {SWeaponSelector("Weapon 1", weapons, setW1, w1())}
      {SWeaponSelector("Weapon 2", weapons, setW2, w2())}
      {SWeaponSelector("Weapon 3", weapons, setW3, w3())}
    </div>
    <div class="row">
      {SWeaponSelector("Weapon 4", weapons, setW4, w4())}
      {SWeaponSelector("Weapon 5", weapons, setW5, w5())}
      {SWeaponSelector("Weapon 6", weapons, setW6, w6())}
    </div>
  </div >
}

const [fhead, setFhead] = createSignal("Any")
const [fbody, setFbody] = createSignal("Any")
const [farms, setFarms] = createSignal("Any")
const [flegs, setFlegs] = createSignal("Any")

function get_forced(): ArmorSlots<string> {
  return {
    Arms: farms(),
    Body: fbody(),
    Head: fhead(),
    Legs: flegs(),
  }
}

function ArmorSelector(armors: Armors): JSX.Element {

  const aselect = (nm: string, armorset: Armor[], setter: Setter<string>, selected: string) => {
    return <div class="col">
      <select class="form-select" id={"as_" + nm} onChange={(op) => {
        const aname = op.currentTarget.value
        localStorage.setItem(nm, aname)
        setter(aname)
      }}>
        <option value="Any">Any</option>
        <For each={armorset}>{(armor) => {
          return <option value={armor.name} selected={armor.name === selected}>
            {armor.name}
          </option>
        }}
        </For>
      </select>
    </div>
  }

  onMount(() => {
    const h = localStorage.getItem("forced_head")
    if (h !== null) {
      setFhead(h)
    }
    const b = localStorage.getItem("forced_body")
    if (b !== null) {
      setFbody(b)
    }
    const a = localStorage.getItem("forced_arms")
    if (a !== null) {
      setFarms(a)
    }
    const l = localStorage.getItem("forced_legs")
    if (l !== null) {
      setFlegs(l)
    }
  })

  return <div class="row">
    <h4>Equipped armor (forced)</h4>
    <div class="row">
      {aselect("forced_head", armors.Head, setFhead, fhead())}
      {aselect("forced_body", armors.Body, setFbody, fbody())}
      {aselect("forced_arms", armors.Arms, setFarms, farms())}
      {aselect("forced_legs", armors.Legs, setFlegs, flegs())}
    </div>
  </div>
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
      <td> {Number(r.score).toFixed(0)} </td>
    </tr>
  }
}

function best_calc(budget: number, weights: Weights<SPair<number>>, mins: Weights<SPair<number>>, armors: Armors, forced: ArmorSlots<string>, all: boolean = false): JSX.Element {
  const nweights: Weights<number> = weights_map(weights, (x) => x.access())
  const nmin: Weights<number> = weights_map(mins, (x) => x.access())
  let r = compute_best(budget, nweights, nmin, armors, forced, all)

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
        <th scope="col">Sc.</th>
      </tr>
    </thead>
    <tbody>
      {best_col(r.Head)}
      {best_col(r.Body)}
      {best_col(r.Arms)}
      {best_col(r.Legs)}
      {best_col(selection_total(r))}
    </tbody>
  </table>
}

function App() {
  const [filter, setFilter] = createSignal("")
  const [weightmode, setWeightmode] = createSignal("70%")
  const [totalWeight, setTotalWeight] = createSignal(45)
  const [extraWeight, setExtraWeight] = createSignal(0)
  const [bestShown, setBestShown] = createSignal(false)
  const weights = defaultWeights();
  const mins = defaultMins();
  onMount(() => {
    const tw = localStorage.getItem("totalWeight")
    if (tw !== null) {
      setTotalWeight(JSON.parse(tw))
    }
    const ew = localStorage.getItem("extraWeight")
    if (ew !== null) {
      setExtraWeight(JSON.parse(ew))
    }
  })

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

      <div class="accordion">
        <div class="accordion-item">
          <h2 class="accordion-header">
            <div class="accordion-button">
              Theoritical best (if all armors were available)
            </div>
          </h2>

          <div class="accordion-collapse collapse show">
            <div class="accordion-body">
              {
                best_calc(weight_budget(weightmode(), totalWeight(), weapons_weight() + extraWeight()), weights, mins, armors, get_forced(), true)
              }
            </div>
          </div>
        </div>
      </div>
      <div class="row">
        {
          best_calc(weight_budget(weightmode(), totalWeight(), weapons_weight() + extraWeight()), weights, mins, armors, get_forced())
        }
      </div>

      <div class="row">
        <div class="col">
          <label for="total_weight" class="form-label">Carrying capacity</label>
          <input class="form-control" id="total_weight" type="number" step="0.1" aria-describedby="total_weight_help" value={totalWeight()}
            onInput={(e) => {
              const v = parseFloat(e.currentTarget.value)
              localStorage.setItem("totalWeight", JSON.stringify(v))
              setTotalWeight(v)
            }} />
          <div id="total_weight_help" class="form-text">Total weight you can bear</div>
        </div>
        <div class="col">
          <label for="extra_weight" class="form-label">Extra weight (charms)</label>
          <input class="form-control" id="extra_weight" type="number" step="0.1" aria-describedby="extra_weight_help" value={extraWeight()}
            onInput={(e) => {
              const v = parseFloat(e.currentTarget.value)
              localStorage.setItem("extraWeight", JSON.stringify(v))
              setExtraWeight(v)
            }} />
          <div id="extra_weight_help" class="form-text">Extra weight such as charms</div>
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
          <input class="form-control" id="wbudget" disabled value={Number(weight_budget(weightmode(), totalWeight(), weapons_weight() + extraWeight())).toFixed(1)} />
        </div>
      </div>
      {ArmorSelector(armors)}
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
    </div >
  );
};

export default App;