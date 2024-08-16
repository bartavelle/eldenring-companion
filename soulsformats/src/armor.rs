use std::collections::{BTreeMap, HashMap};

use eldenring_companion::{Absorptions, Armor, ArmorCategory, Resistances};

use crate::{
    formats::{bnd4::BND4, load_params, load_params_filter},
    structs::{
        ds1r::{self, reinforce_param_protector::REINFORCE_PARAM_PROTECTOR_ST},
        equip_param_protector::EQUIP_PARAM_PROTECTOR_ST,
    },
};

fn convx(i: f32) -> f32 {
    let p1000 = (i * 1000.0).round();
    (1000.0 - p1000) / 10.0
}

const UNOBTAINABLE: [&str; 1] = ["Deathbed Smalls"];

pub fn load_armor(reg: &BND4, armor_names: &HashMap<u32, String>) -> anyhow::Result<BTreeMap<u32, Armor>> {
    load_params_filter(
        reg,
        "EquipParamProtector",
        |rid, _nm, raw_armor: EQUIP_PARAM_PROTECTOR_ST| {
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
                weight: raw_armor.weight,
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
                    focus: raw_armor.resist_madness as f32,
                    immunity: raw_armor.resist_disease as f32,
                    poise: raw_armor.toughness_correct_rate * 1000.0,
                    robustness: raw_armor.resist_blood as f32,
                    vitality: raw_armor.resist_curse as f32,
                },
            };
            Some((rid, armor))
        },
    )
}

pub fn load_armor_ds1(reg: &BND4, names: &HashMap<u32, String>) -> anyhow::Result<BTreeMap<u32, Armor>> {
    let reinf = load_params(
        reg,
        "ReinforceParamProtector",
        |rid, _, raw_p: REINFORCE_PARAM_PROTECTOR_ST| (rid, raw_p),
    )?;

    load_params_filter(
        reg,
        "EquipParamProtector",
        |rid, _, raw_armor: ds1r::equip_param_protector::EQUIP_PARAM_PROTECTOR_ST| {
            if rid < 10000 {
                return None;
            }
            let name = names.get(&rid).map(|s| s.as_str())?;
            if UNOBTAINABLE.contains(&name) {
                return None;
            }
            let category = match raw_armor.equip_model_category {
                2 => ArmorCategory::Body,
                6 => ArmorCategory::Legs,
                1 => ArmorCategory::Arms,
                5 => ArmorCategory::Head,
                _ => panic!("invalid body type {rid}/{name} {raw_armor:?}"),
            };

            let max_reinf = match raw_armor.reinforce_type_id {
                0 => reinf.get(&10),
                100 => reinf.get(&105),
                n => panic!("unsupported reinforcement id {n} for {name} [{rid}]"),
            }
            .unwrap();

            let physical = raw_armor.defense_physics as f32 * max_reinf.physics_def_rate;
            let rate = |x: i16| physical * (100.0 + x as f32) / 100.0;

            let armor = Armor {
                category,
                name: name.to_string(),
                weight: raw_armor.weight,
                absorptions: Absorptions {
                    fire: raw_armor.defense_fire as f32 * max_reinf.fire_def_rate / 10.0,
                    holy: 0.0,
                    lightning: raw_armor.defense_thunder as f32 * max_reinf.thunder_def_rate / 10.0,
                    magic: raw_armor.defense_magic as f32 * max_reinf.magic_def_rate / 10.0,
                    physical: physical/ 10.0,
                    pierce: rate(raw_armor.defense_thrust) * max_reinf.thrust_def_rate / 10.0,
                    slash: rate(raw_armor.defense_slash) * max_reinf.slash_def_rate/10.0,
                    strike: rate(raw_armor.defense_blow) * max_reinf.blow_def_rate/10.0,
                },
                resistances: Resistances {
                    focus: raw_armor.resist_poison as f32 * max_reinf.resist_poison_rate,
                    immunity: raw_armor.resist_disease as f32 * max_reinf.resist_disease_rate,
                    poise: raw_armor.sa_durability as f32,
                    robustness: raw_armor.resist_blood as f32 * max_reinf.resist_blood_rate,
                    vitality: raw_armor.resist_curse as f32 * max_reinf.resist_curse_rate,
                },
            };
            Some((rid, armor))
        },
    )
}
