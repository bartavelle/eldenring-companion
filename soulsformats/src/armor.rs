use std::collections::{BTreeMap, HashMap};

use eldenring_companion::{Absorptions, Armor, ArmorCategory, Resistances};

use crate::{
    formats::{bnd4::BND4, load_params_filter},
    structs::equip_param_protector::EQUIP_PARAM_PROTECTOR_ST,
};

fn conv(i: f32) -> f64 {
    ((i * 10.0) as f64) / 10.0
}

fn convx(i: f32) -> f64 {
    let p1000 = (i * 1000.0).round() as f64;
    (1000.0 - p1000) / 10.0
}

const UNOBTAINABLE: [&str; 1] = ["Deathbed Smalls"];

pub fn load_armor(reg: &BND4, armor_names: &HashMap<u32, String>) -> anyhow::Result<BTreeMap<u32, Armor>> {
    load_params_filter(
        reg,
        "EquipParamProtector",
        |rid, _, raw_armor: EQUIP_PARAM_PROTECTOR_ST| {
            if rid < 40000 {
                return None;
            }
            let name = armor_names.get(&rid)?;
            if UNOBTAINABLE.contains(&name.as_str()) {
                return None;
            }
            let category = match raw_armor.equip_model_category {
                2 => ArmorCategory::Body,
                6 => ArmorCategory::Legs,
                1 => ArmorCategory::Arms,
                5 => ArmorCategory::Head,
                _ => panic!("invalid body type {rid}/{name} {raw_armor:?}"),
            };

            let armor = Armor {
                category,
                name: name.to_string(),
                weight: conv(raw_armor.weight),
                absorptions: Absorptions {
                    fire: convx(raw_armor.fire_damage_cut_rate),
                    holy: convx(raw_armor.dark_damage_cut_rate),
                    lightning: convx(raw_armor.thunder_damage_cut_rate),
                    magic: convx(raw_armor.magic_damage_cut_rate),
                    physical: convx(raw_armor.neutral_damage_cut_rate),
                    pierce: convx(raw_armor.thrust_damage_cut_rate),
                    slash: convx(raw_armor.slash_damage_cut_rate),
                    strike: convx(raw_armor.blow_damage_cut_rate),
                },
                resistances: Resistances {
                    focus: raw_armor.resist_madness as f64,
                    immunity: raw_armor.resist_disease as f64,
                    poise: conv(raw_armor.toughness_correct_rate * 1000.0),
                    robustness: raw_armor.resist_blood as f64,
                    vitality: raw_armor.resist_curse as f64,
                },
            };
            Some((rid, armor))
        },
    )
}
