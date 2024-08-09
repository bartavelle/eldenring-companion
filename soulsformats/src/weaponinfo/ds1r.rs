use std::collections::{BTreeMap, HashMap};

use crate::{
    formats::{bnd4::BND4, load_params},
    scaling::Scaling,
    structs::ds1r::{
        calc_correct_graph::CACL_CORRECT_GRAPH_ST, equip_param_weapon::EQUIP_PARAM_WEAPON_ST,
        reinforce_param_weapon::REINFORCE_PARAM_WEAPON_ST, sp_effect::SP_EFFECT_PARAM_ST,
    },
    weaponinfo::PassiveLvl,
};
use ertypes::stats::{Damage, Effect, Passive, Reinforcement, Stat};
use packed_struct::PackedStructSlice;

use super::{SpEffect, WeaponData, WeaponInfo};

fn load_sp_effects(reg: &BND4) -> anyhow::Result<BTreeMap<u32, SpEffect>> {
    load_params(reg, "SpEffect", |rid, name, a: SP_EFFECT_PARAM_ST| {
        (
            rid,
            SpEffect {
                poizon_attack_power: a.poizon_attack_power as f32,
                disease_attack_power: 0.0,
                blood_attack_power: 0.0,
                freeze_attack_power: 0.0,
                sleep_attack_power: 0.0,
                madness_attack_power: 0.0,
                name: name.map(|x| x.to_string()),
            },
        )
    })
}

fn load_reinforcement(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Reinforcement>> {
    load_params(
        reg,
        "ReinforceParamWeapon",
        |rid, name, rw: REINFORCE_PARAM_WEAPON_ST| {
            let upgrade_level = rid % 100;
            let reinforce_type = rid - upgrade_level;
            let reinforce_damage = Damage {
                physics: rw.physics_atk_rate,
                magic: rw.magic_atk_rate,
                fire: rw.fire_atk_rate,
                lightning: rw.thunder_atk_rate,
                holy: 0.0,
            };
            let reinforce_stats = Stat {
                str: rw.correct_strength_rate,
                dex: rw.correct_agility_rate,
                int: rw.correct_magic_rate,
                fth: rw.correct_faith_rate,
                arc: 0.0,
            };
            (
                reinforce_type,
                Reinforcement {
                    damage: reinforce_damage,
                    stats: reinforce_stats,
                    sp0: rw.sp_effect_id1,
                    sp1: rw.sp_effect_id2,
                    sp2: rw.sp_effect_id3,
                    name: name.map(|x| x.to_string()),
                },
            )
        },
    )
}

fn load_weapons(
    reg: &BND4,
    weapon_names: &HashMap<u32, String>,
    sp_effects: &BTreeMap<u32, SpEffect>,
    reinforce: &BTreeMap<u32, Reinforcement>,
) -> anyhow::Result<Vec<WeaponInfo>> {
    let raw_equip_param_weapon = reg.get_file("EquipParamWeapon").unwrap();
    let equip_param_weapon = crate::formats::params::Params::from_bytes(raw_equip_param_weapon)?;

    let mut weapons = Vec::new();

    for ridx in 0..equip_param_weapon.row_count() {
        let (rid, name, rdata) = equip_param_weapon.raw_row(ridx);

        let infusion_id = rid % 1000;
        let base_id = rid - infusion_id;

        let name = match (infusion_id, weapon_names.get(&base_id)) {
            (0, Some(n)) => n.to_string(),
            (100, Some(n)) => format!("Crystal {n}"),
            (200, Some(n)) => format!("Lightning {n}"),
            (300, Some(n)) => format!("Raw {n}"),
            (400, Some(n)) => format!("Magic {n}"),
            (500, Some(n)) => format!("Occult {n}"),
            (600, Some(n)) => format!("Divine {n}"),
            (700, Some(n)) => format!("Dark {n}"),
            (800, Some(n)) => format!("Fire {n}"),
            (900, Some(n)) => format!("Chaos {n}"),
            _ => name.unwrap_or_default().to_string(),
        };

        // let base_name = weapon_names.get(&base_id).map(|x| x.as_str()).or(name).unwrap_or_default();

        let eqpr = EQUIP_PARAM_WEAPON_ST::unpack_from_slice(rdata)?;
        let wpn = new_weaponinfo(name, rid, &eqpr, sp_effects, reinforce)?;
        weapons.push(wpn);
    }
    Ok(weapons)
}

fn load_correct(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Scaling>> {
    load_params(reg, "CalcCorrectGraph", |rid, nm, clc: CACL_CORRECT_GRAPH_ST| {
        let row = Scaling::new_ds1r(&clc);
        (rid, row)
    })
}

pub fn load_weapondata(reg: &BND4, weapon_names: &HashMap<u32, String>) -> anyhow::Result<WeaponData> {
    let sp_effects = load_sp_effects(reg)?;
    let reinforcement = load_reinforcement(reg)?;
    let graphes = load_correct(reg)?;
    let weapons = load_weapons(reg, weapon_names, &sp_effects, &reinforcement)?;
    Ok(WeaponData {
        sp_effects,
        reinforcement,
        aec: BTreeMap::new(),
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
        mainid: id / 1000,
        attack_base: Damage {
            physics: eqp.attack_base_physics,
            magic: eqp.attack_base_magic,
            fire: eqp.attack_base_fire,
            lightning: eqp.attack_base_thunder,
            holy: 0,
        },
        required: Stat {
            str: eqp.proper_strength,
            dex: eqp.proper_agility,
            int: eqp.proper_magic,
            fth: eqp.proper_faith,
            arc: 0,
        },
        correct_a: Stat {
            str: eqp.correct_strength,
            dex: eqp.correct_agility,
            int: eqp.correct_magic,
            fth: eqp.correct_faith,
            arc: 0.0,
        },
        correct_d: Damage {
            physics: eqp.correct_type,
            magic: eqp.correct_type,
            fire: eqp.correct_type,
            lightning: eqp.correct_type,
            holy: eqp.correct_type,
        },
        correct_e: Effect {
            sleep: eqp.correct_type,
            poison: eqp.correct_type,
            blood: eqp.correct_type,
            madness: eqp.correct_type,
            frost: 0,
            scarlet_rot: 0,
        },
        reinforce_id: eqp.reinforce_type_id,
        correct_id: 0,
        passives,
    })
}
