import { Accessor, For, Setter, Show, createSignal, onMount } from "solid-js";
import armament_dict from "./data/armaments.json";

type Arm = {
  category: string,
  name: string,
  weight: number,
  setOwned: Setter<boolean>,
  owned: Accessor<boolean>
}

let armaments: Arm[] = Object.keys(armament_dict).map(function (wname) {
  let wpn = armament_dict[wname];
  const [c, s] = createSignal(false)
  return {
    category: wpn.category,
    name: wpn.name,
    weight: wpn.weight,
    setOwned: s,
    owned: c
  };
});

function save_owned() {
  let owned_map: { [id: string]: boolean } = {}
  for (let a of armaments) {
    owned_map[a.name] = a.owned()
  }
  localStorage.setItem("armaments", JSON.stringify(owned_map))
}

function load_owned() {
  let x = localStorage.getItem("armaments")
  if (x === null) {
    console.log("no previous storage")
    return
  }
  let owned_map: { [id: string]: boolean } = JSON.parse(x)
  for (let a of armaments) {
    a.setOwned(!!owned_map[a.name])
  }
}

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

const Armament = (filter: string) => {
  onMount(async () => {
    load_owned()
  })
  return <div>
    <For each={armaments}>{(ow) =>
      <Show when={ow.name.toLowerCase().includes(filter.toLowerCase())}>
        <button type="button" class={button_class(ow.owned())} onClick={() => {
          ow.setOwned(!ow.owned())
          save_owned()
        }}>
          {ow.name}
        </button>
      </Show>
    }</For>
  </div >
};

export default Armament;
export { type Arm, armaments };