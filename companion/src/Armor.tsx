import { Accessor, For, Setter, Show, createSignal } from "solid-js";
import armor_dict from "./data/armor.json";

type Armor = {
  category: "Legs" | "Body" | "Head" | "Arms",
  name: string,
  weight: number,
  setOwned: Setter<boolean>,
  owned: Accessor<boolean>
  absorptions: {
    fire: number,
    holy: number,
    lightning: number,
    magic: number,
    physical: number,
    pierce: number,
    slash: number,
    strike: number
  },
  resistances: {
    focus: number,
    immunity: number,
    poise: number,
    robustness: number,
    vitality: number
  },
}

type Armors = {
  Arms: Armor[]
  Legs: Armor[]
  Body: Armor[]
  Head: Armor[]
}


let armors: Armors = {
  Arms: [],
  Legs: [],
  Body: [],
  Head: [],
}

for (let aname of Object.keys(armor_dict)) {
  let armor = armor_dict[aname];
  const [c, s] = createSignal(false)
  let t = armors.Arms
  switch (armor.category) {
    case "Legs":
      t = armors.Legs
      break;
    case "Arms":
      t = armors.Arms
      break;
    case "Body":
      t = armors.Body
      break;
    case "Head":
      t = armors.Head
      break;
    default:
      console.log("invalid category", armor.category)
      break;
  }
  t.push({
    category: armor.category,
    name: armor.name,
    weight: armor.weight,
    setOwned: s,
    owned: c,
    absorptions: armor.absorptions,
    resistances: armor.resistances
  });
}

const sorter = (a: Armor, b: Armor) => {
  if (a.name < b.name) {
    return -1
  } else if (a.name > b.name) {
    return 1
  } else {
    return 0
  }
}

armors.Head.sort(sorter)
armors.Arms.sort(sorter)
armors.Body.sort(sorter)
armors.Legs.sort(sorter)

function button_class(active: boolean) {
  if (active) {
    return "btn btn-primary"
  } else {
    return "btn btn-light"
  }
}

const ACol = (armors: Armor[], filter: string) => {
  return <div class="btn-group-vertical" role="group">
    <For each={armors}>{(ow) =>
      <Show when={ow.name.toLowerCase().includes(filter.toLowerCase())}>
        <button type="button" class={button_class(ow.owned())} onClick={() => {
          ow.setOwned(!ow.owned())
        }}>{ow.name}</button>
      </Show>
    }</For>
  </div>
}

const set_all_owned = () => {
  const suball = (arr: Armor[]) => {
    for (let a of arr) {
      a.setOwned(true)
    }
  }
  suball(armors.Head)
  suball(armors.Arms)
  suball(armors.Body)
  suball(armors.Legs)
}

const Armor = (filter: string) => {
  return <div>
    <div class="row align-items-start">
      <button class="btn btn-danger" onClick={set_all_owned}>Set all armors to owned</button>
    </div>
    <div class="row align-items-start">
      <div class="col">
        {ACol(armors.Head, filter)}
      </div>
      <div class="col">
        {ACol(armors.Arms, filter)}
      </div>
      <div class="col">
        {ACol(armors.Body, filter)}
      </div>
      <div class="col">
        {ACol(armors.Legs, filter)}
      </div>
    </div >
  </div >
};

export default Armor;
export { type Armors, armors }