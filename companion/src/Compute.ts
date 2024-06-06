import Armor, { Armors } from "./Armor";
import { Weights } from "./WSelector";
import { ArmorSlots, Azipwith } from "./Common"

type ALst = {
  values: Weights<number>,
  n: string,
  score: number,
  weight: number
}

function prepare_armor(weights: Weights<number>, lst: Armor[], forced: string, all: boolean): ALst[] {
  let tmp: ALst[] = [];
  for (let a of lst) {
    if (!(all || a.owned()) && a.name !== forced)
      continue;
    const score = weights.absorptions.fire * a.absorptions.fire +
      weights.absorptions.holy * a.absorptions.holy +
      weights.absorptions.lightning * a.absorptions.lightning +
      weights.absorptions.magic * a.absorptions.magic +
      weights.absorptions.physical * a.absorptions.physical +
      weights.absorptions.pierce * a.absorptions.pierce +
      weights.absorptions.slash * a.absorptions.slash +
      weights.absorptions.strike * a.absorptions.strike +
      weights.resistances.focus * a.resistances.focus +
      weights.resistances.poise * a.resistances.poise +
      weights.resistances.robustness * a.resistances.robustness +
      weights.resistances.vitality * a.resistances.vitality +
      weights.resistances.immunity * a.resistances.immunity
    tmp.push({
      values: {
        absorptions: a.absorptions,
        resistances: a.resistances,
      },
      n: a.name,
      score: score,
      weight: a.weight
    })
  }
  tmp.sort((a, b) => {
    if (a.score > b.score) {
      return 1
    } else if (a.score < b.score) {
      return -1
    } else {
      return 0
    }
  })
  tmp.reverse()

  let out: ALst[] = [];
  let curel: ALst | null = null
  for (let a of tmp) {
    if (curel === null) {
      curel = a
    } else {
      if (a.n !== forced && curel.weight <= a.weight) {
        continue
      }
      out.push(curel)
      curel = a
    }
  }
  if (curel !== null) {
    out.push(curel)
  }
  out.push({
    values: {
      absorptions: {
        fire: 0,
        holy: 0,
        lightning: 0,
        magic: 0,
        physical: 0,
        pierce: 0,
        slash: 0,
        strike: 0
      },
      resistances: {
        focus: 0,
        immunity: 0,
        poise: 0,
        robustness: 0,
        vitality: 0
      }
    },
    n: "Empty",
    score: 0,
    weight: 0
  })

  return out
}

type Selection = ArmorSlots<ALst | null>

function compute_best(budget: number, weights: Weights<number>, mins: Weights<number>, armors: Armors, forced: ArmorSlots<string>, all: boolean = false): Selection {

  let prepared = Azipwith((ar, fo) => {
    let o = prepare_armor(weights, ar, fo, all)
    // forced items
    if (fo !== "Any") {
      o = o.filter((a) => a.n == fo)
    }
    return o
  }, armors, forced)

  let best_score = 0
  let best_selection: Selection = {
    Arms: null,
    Body: null,
    Head: null,
    Legs: null,
  }
  const reach_minimal = (values: Weights<number>) => {
    return values.absorptions.fire >= mins.absorptions.fire
      && values.absorptions.holy >= mins.absorptions.holy
      && values.absorptions.lightning >= mins.absorptions.lightning
      && values.absorptions.magic >= mins.absorptions.magic
      && values.absorptions.physical >= mins.absorptions.physical
      && values.absorptions.pierce >= mins.absorptions.pierce
      && values.absorptions.slash >= mins.absorptions.slash
      && values.absorptions.strike >= mins.absorptions.strike
      && values.resistances.focus >= mins.resistances.focus
      && values.resistances.immunity >= mins.resistances.immunity
      && values.resistances.poise >= mins.resistances.poise
      && values.resistances.robustness >= mins.resistances.robustness
      && values.resistances.vitality >= mins.resistances.vitality

  }
  for (let b of prepared.Body) {
    let bw = budget - b.weight
    if (bw < 0) {
      continue
    }
    const values = selection_total({ Arms: null, Body: b, Head: null, Legs: null }).values
    if (reach_minimal(values)) {
      const curscore = b.score
      if (curscore > best_score) {
        best_score = curscore
        best_selection = {
          Arms: null,
          Body: b,
          Head: null,
          Legs: null
        }
      }
    }
    for (let l of prepared.Legs) {
      let bl = bw - l.weight
      if (bl < 0) {
        continue
      }
      const values = selection_total({ Arms: null, Body: b, Head: null, Legs: l }).values
      if (reach_minimal(values)) {
        const curscore = b.score + l.score
        if (curscore > best_score) {
          best_score = curscore
          best_selection = {
            Arms: null,
            Body: b,
            Head: null,
            Legs: l
          }
        }
      }
      for (let h of prepared.Head) {
        let bh = bl - h.weight
        if (bh < 0) {
          continue
        }
        const values = selection_total({ Arms: null, Body: b, Head: h, Legs: l }).values
        if (reach_minimal(values)) {
          const curscore = b.score + l.score + h.score
          if (curscore > best_score) {
            best_score = curscore
            best_selection = {
              Arms: null,
              Body: b,
              Head: h,
              Legs: l
            }
          }
        }
        for (let a of prepared.Arms) {
          let ba = bh - a.weight
          if (ba < 0) {
            continue
          }
          let values = selection_total({ Arms: a, Body: b, Head: h, Legs: l }).values
          if (!reach_minimal(values)) {
            continue
          }
          const curscore = b.score + l.score + h.score + a.score
          if (curscore > best_score) {
            best_score = curscore
            best_selection = {
              Arms: a,
              Body: b,
              Head: h,
              Legs: l
            }
          }
          break
        }
      }
    }
  }
  return best_selection
}

function selection_total(s: Selection): ALst {
  type GF = (a: ALst) => number;
  const v = (getter: GF) => {
    let t = 0
    if (s.Arms !== null) {
      t += getter(s.Arms)
    }
    if (s.Body !== null) {
      t += getter(s.Body)
    }
    if (s.Legs !== null) {
      t += getter(s.Legs)
    }
    if (s.Head !== null) {
      t += getter(s.Head)
    }
    return t
  }

  return {
    n: "Total",
    score: v((x) => x.score),
    weight: v((x) => x.weight),
    values: {
      absorptions: {
        fire: v((x) => x.values.absorptions.fire),
        holy: v((x) => x.values.absorptions.holy),
        lightning: v((x) => x.values.absorptions.lightning),
        magic: v((x) => x.values.absorptions.magic),
        physical: v((x) => x.values.absorptions.physical),
        slash: v((x) => x.values.absorptions.slash),
        pierce: v((x) => x.values.absorptions.pierce),
        strike: v((x) => x.values.absorptions.strike),
      },
      resistances: {
        focus: v((x) => x.values.resistances.focus),
        poise: v((x) => x.values.resistances.poise),
        immunity: v((x) => x.values.resistances.immunity),
        robustness: v((x) => x.values.resistances.robustness),
        vitality: v((x) => x.values.resistances.vitality),
      }

    }
  }

}

export default compute_best
export { type Selection, type ALst, type ArmorSlots, selection_total }
