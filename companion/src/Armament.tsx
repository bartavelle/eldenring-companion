import { Accessor, For, Setter, Show, createSignal } from "solid-js";
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
  return <div>
    <For each={armaments}>{(ow) =>
      <Show when={ow.name.toLowerCase().includes(filter.toLowerCase())}>
        <button type="button" class={button_class(ow.owned())} onClick={() => {
          ow.setOwned(!ow.owned())
        }}>
          {ow.name}
        </button>
      </Show>
    }</For>
  </div >
};

export default Armament;
export { type Arm, armaments };