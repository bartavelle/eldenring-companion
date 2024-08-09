use std::collections::{BTreeMap, HashMap};

use crate::{
    formats::{bnd4::BND4, load_params},
    scaling::Scaling,
    structs::{
        attack_element_correct::ATTACK_ELEMENT_CORRECT_PARAM_ST, calc_correct_graph::CACL_CORRECT_GRAPH_ST,
        equip_param_weapon::EQUIP_PARAM_WEAPON_ST, reinforce_param_weapon::REINFORCE_PARAM_WEAPON_ST,
        sp_effect::SP_EFFECT_PARAM_ST,
    },
    weaponinfo::PassiveLvl,
};
use ertypes::stats::{Damage, Effect, Passive, Reinforcement, Stat};
use packed_struct::PackedStructSlice;

use super::{SpEffect, WeaponData, WeaponInfo};

fn load_sp_effects(reg: &BND4) -> anyhow::Result<BTreeMap<u32, SpEffect>> {
    load_params(reg, "SpEffect", |rid, _, a: SP_EFFECT_PARAM_ST| {
        (
            rid,
            SpEffect {
                poizon_attack_power: a.poizon_attack_power as f32,
                disease_attack_power: a.disease_attack_power as f32,
                blood_attack_power: a.blood_attack_power as f32,
                freeze_attack_power: a.freeze_attack_power as f32,
                sleep_attack_power: a.sleep_attack_power as f32,
                madness_attack_power: a.madness_attack_power as f32,
                name: None,
            },
        )
    })
}

fn load_reinforcement(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Reinforcement>> {
    load_params(reg, "ReinforceParamWeapon", |rid, _, rw: REINFORCE_PARAM_WEAPON_ST| {
        let upgrade_level = rid % 100;
        let reinforce_type = rid - upgrade_level;
        let reinforce_damage = Damage {
            physics: rw.physics_atk_rate,
            magic: rw.magic_atk_rate,
            fire: rw.fire_atk_rate,
            lightning: rw.thunder_atk_rate,
            holy: rw.dark_atk_rate,
        };
        let reinforce_stats = Stat {
            str: rw.correct_strength_rate,
            dex: rw.correct_agility_rate,
            int: rw.correct_magic_rate,
            fth: rw.correct_faith_rate,
            arc: rw.correct_luck_rate,
        };
        (
            reinforce_type,
            Reinforcement {
                damage: reinforce_damage,
                stats: reinforce_stats,
                sp0: rw.sp_effect_id1,
                sp1: rw.sp_effect_id2,
                sp2: rw.sp_effect_id3,
                name: None,
            },
        )
    })
}

fn load_aec(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Stat<Damage<i16>>>> {
    load_params(
        reg,
        "AttackElementCorrectParam",
        |rid, _, rw: ATTACK_ELEMENT_CORRECT_PARAM_ST| {
            let d = |rate: i16, iscorrected: bool| if iscorrected { rate } else { 0 };
            let correct = Stat {
                str: Damage {
                    physics: d(
                        rw.influence_strength_correct_rate_by_physics,
                        rw.is_strength_correct_by_physics,
                    ),
                    magic: d(
                        rw.influence_strength_correct_rate_by_magic,
                        rw.is_strength_correct_by_magic,
                    ),
                    fire: d(
                        rw.influence_strength_correct_rate_by_fire,
                        rw.is_strength_correct_by_fire,
                    ),
                    lightning: d(
                        rw.influence_strength_correct_rate_by_thunder,
                        rw.is_strength_correct_by_thunder,
                    ),
                    holy: d(
                        rw.influence_strength_correct_rate_by_dark,
                        rw.is_strength_correct_by_dark,
                    ),
                },
                dex: Damage {
                    physics: d(
                        rw.influence_dexterity_correct_rate_by_physics,
                        rw.is_dexterity_correct_by_physics,
                    ),
                    magic: d(
                        rw.influence_dexterity_correct_rate_by_magic,
                        rw.is_dexterity_correct_by_magic,
                    ),
                    fire: d(
                        rw.influence_dexterity_correct_rate_by_fire,
                        rw.is_dexterity_correct_by_fire,
                    ),
                    lightning: d(
                        rw.influence_dexterity_correct_rate_by_thunder,
                        rw.is_dexterity_correct_by_thunder,
                    ),
                    holy: d(
                        rw.influence_dexterity_correct_rate_by_dark,
                        rw.is_dexterity_correct_by_dark,
                    ),
                },
                int: Damage {
                    physics: d(
                        rw.influence_magic_correct_rate_by_physics,
                        rw.is_magic_correct_by_physics,
                    ),
                    magic: d(rw.influence_magic_correct_rate_by_magic, rw.is_magic_correct_by_magic),
                    fire: d(rw.influence_magic_correct_rate_by_fire, rw.is_magic_correct_by_fire),
                    lightning: d(
                        rw.influence_magic_correct_rate_by_thunder,
                        rw.is_magic_correct_by_thunder,
                    ),
                    holy: d(rw.influence_magic_correct_rate_by_dark, rw.is_magic_correct_by_dark),
                },
                fth: Damage {
                    physics: d(
                        rw.influence_faith_correct_rate_by_physics,
                        rw.is_faith_correct_by_physics,
                    ),
                    magic: d(rw.influence_faith_correct_rate_by_magic, rw.is_faith_correct_by_magic),
                    fire: d(rw.influence_faith_correct_rate_by_fire, rw.is_faith_correct_by_fire),
                    lightning: d(
                        rw.influence_faith_correct_rate_by_thunder,
                        rw.is_faith_correct_by_thunder,
                    ),
                    holy: d(rw.influence_faith_correct_rate_by_dark, rw.is_faith_correct_by_dark),
                },
                arc: Damage {
                    physics: d(rw.influence_luck_correct_rate_by_physics, rw.is_luck_correct_by_physics),
                    magic: d(rw.influence_luck_correct_rate_by_magic, rw.is_luck_correct_by_magic),
                    fire: d(rw.influence_luck_correct_rate_by_fire, rw.is_luck_correct_by_fire),
                    lightning: d(rw.influence_luck_correct_rate_by_thunder, rw.is_luck_correct_by_thunder),
                    holy: d(rw.influence_luck_correct_rate_by_dark, rw.is_luck_correct_by_dark),
                },
            };
            (rid, correct)
        },
    )
}

fn load_weapons(
    reg: &BND4,
    weapons_names: &HashMap<u32, String>,
    sp_effects: &BTreeMap<u32, SpEffect>,
    reinforce: &BTreeMap<u32, Reinforcement>,
) -> anyhow::Result<Vec<WeaponInfo>> {
    let raw_equip_param_weapon = reg.get_file("EquipParamWeapon").unwrap();
    let equip_param_weapon = crate::formats::params::Params::from_bytes(raw_equip_param_weapon)?;

    let mut weapons = Vec::new();

    for ridx in 0..equip_param_weapon.row_count() {
        let (rid, _, rdata) = equip_param_weapon.raw_row(ridx);
        if rid == 99999999 {
            continue;
        }
        if rid < 1000000 {
            // items
            continue;
        }
        if (30000000..33000000).contains(&rid) {
            // shields
            continue;
        }

        if (50000000..53500000).contains(&rid) {
            // arrows
            continue;
        }
        let eqpr = EQUIP_PARAM_WEAPON_ST::unpack_from_slice(rdata)?;
        if let Some(nm) = weapons_names.get(&rid) {
            let wpn = new_weaponinfo(nm.clone(), rid, &eqpr, sp_effects, reinforce)?;
            weapons.push(wpn);
        }
    }
    Ok(weapons)
}

fn load_correct(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Scaling>> {
    load_params(reg, "CalcCorrectGraph", |rid, _, clc: CACL_CORRECT_GRAPH_ST| {
        let row = Scaling::new(&clc);
        (rid, row)
    })
}

pub fn load_weapondata(reg: &BND4, names: &HashMap<u32, String>) -> anyhow::Result<WeaponData> {
    let sp_effects = load_sp_effects(reg)?;
    let reinforcement = load_reinforcement(reg)?;
    let aec = load_aec(reg)?;
    let graphes = load_correct(reg)?;
    let weapons = load_weapons(reg, names, &sp_effects, &reinforcement)?;
    Ok(WeaponData {
        sp_effects,
        reinforcement,
        aec,
        graphes,
        weapons,
    })
}

pub fn new_weaponinfo(
    name: String,
    id: u32,
    eqp: &EQUIP_PARAM_WEAPON_ST,
    sp: &BTreeMap<u32, SpEffect>,
    reinforce: &BTreeMap<u32, Reinforcement>,
) -> anyhow::Result<WeaponInfo> {
    let mut passives = Vec::new();

    let reinforce = reinforce.get(&(eqp.reinforce_type_id as u32)).unwrap();
    for (id, effect) in [
        eqp.sp_effect_behavior_id0 + reinforce.sp0 as i32,
        eqp.sp_effect_behavior_id1 + reinforce.sp1 as i32,
        eqp.sp_effect_behavior_id2 + reinforce.sp2 as i32,
    ]
    .into_iter()
    .enumerate()
    {
        if effect < 0 {
            continue;
        }
        /*
         apparently sp_attribute is set according to the passive type:
          * 20 -> poison
          * 21 -> disease
          * 22 -> blood
          * 23 -> cold
          * 24 -> sleep
          * 25 -> madness
        */

        let i = sp.get(&(effect as u32)).unwrap();
        assert!(
            match eqp.sp_attribute {
                20 => i.poizon_attack_power,
                21 => i.disease_attack_power,
                22 => i.blood_attack_power,
                23 => i.freeze_attack_power,
                24 => i.sleep_attack_power,
                25 => i.madness_attack_power,
                _ => 1.0,
            } > 0.0
        );
        let btp = if i.blood_attack_power > 0.0 {
            Some((i.blood_attack_power, Passive::Blood))
        } else if i.sleep_attack_power > 0.0 {
            Some((i.sleep_attack_power, Passive::Sleep))
        } else if i.freeze_attack_power > 0.0 {
            Some((i.freeze_attack_power, Passive::Frost))
        } else if i.disease_attack_power > 0.0 {
            Some((i.disease_attack_power, Passive::ScarletRot))
        } else if i.madness_attack_power > 0.0 {
            Some((i.madness_attack_power, Passive::Madness))
        } else if i.poizon_attack_power > 0.0 {
            Some((i.poizon_attack_power, Passive::Poison))
        } else {
            None
        };
        if let Some((base, tp)) = btp {
            passives.push(PassiveLvl { id: id as u8, tp, base });
        }
    }
    Ok(WeaponInfo {
        name,
        weight: eqp.weight,
        id,
        mainid: id / 10000,
        attack_base: Damage {
            physics: eqp.attack_base_physics,
            magic: eqp.attack_base_magic,
            fire: eqp.attack_base_fire,
            lightning: eqp.attack_base_thunder,
            holy: eqp.attack_base_dark,
        },
        required: Stat {
            str: eqp.proper_strength,
            dex: eqp.proper_agility,
            int: eqp.proper_magic,
            fth: eqp.proper_faith,
            arc: eqp.proper_luck,
        },
        correct_a: Stat {
            str: eqp.correct_strength,
            dex: eqp.correct_agility,
            int: eqp.correct_magic,
            fth: eqp.correct_faith,
            arc: eqp.correct_luck,
        },
        correct_d: Damage {
            physics: eqp.correct_typephysics,
            magic: eqp.correct_typemagic,
            fire: eqp.correct_typefire,
            lightning: eqp.correct_typethunder,
            holy: eqp.correct_typedark,
        },
        correct_e: Effect {
            sleep: eqp.correct_typesleep,
            poison: eqp.correct_typepoison,
            blood: eqp.correct_typeblood,
            madness: eqp.correct_typemadness,
            frost: 0,
            scarlet_rot: 0,
        },
        reinforce_id: eqp.reinforce_type_id,
        correct_id: eqp.attack_element_correct_id,
        passives,
    })
}
