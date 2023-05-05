import { Accessor, Component, For, Setter, Show, createSignal, onMount } from "solid-js";
import armament_dict from "./data/armaments.json";

type Arm = {
  category: string,
  name: string,
  weight: number
}

let rcategories: { [id: string]: boolean } = {};
let armaments: Arm[] = Object.keys(armament_dict).map(function (wname) {
  let wpn = armament_dict[wname];
  rcategories[wpn.category] = true
  return {
    category: wpn.category,
    name: wpn.name,
    weight: wpn.weight
  };
});

armaments.sort((a: Arm, b: Arm) => {
  if (a.category < b.category) {
    return -1
  } else if (a.category > b.category) {
    return 1
  } else if (a.name < b.name) {
    return -1
  } else if (a.name > b.name) {
    return 1
  } else {
    return 0
  }
})

function button_class(active: boolean) {
  if (active) {
    return "btn btn-primary"
  } else {
    return "btn btn-light"
  }
}

const Armament: Component = () => {
  let ncategories: { [id: string]: { cat: Accessor<boolean>, set: Setter<boolean>, name: string } } = {}
  for (let cat of Object.keys(rcategories)) {
    const [c, s] = createSignal(true);
    ncategories[cat] = { cat: c, set: s, name: cat }
  }
  console.log(ncategories)

  return <div>
    <For each={Object.keys(ncategories)}>{(cat) => {
      var t = ncategories[cat];
      return <button type="button" class={button_class(t.cat())} onClick={() => {
        t.set(!t.cat())
      }}>
        {cat}
      </button>
    }}
    </For>
    <ul>
      <For each={armaments}>{(ow) =>
        <Show when={ncategories[ow.category].cat()}><li>{ow.name}</li></Show>
      }</For>
    </ul>
  </div >
};

export default Armament;