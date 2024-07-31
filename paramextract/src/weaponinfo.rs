use std::collections::{BTreeMap, HashMap};

use anyhow::Context;
use packed_struct::PackedStructSlice;
use serde::Serialize;

use crate::{
    erformat::{bnd4::BND4, load_params},
    scaling::Scaling,
    structs::{
        attack_element_correct::ATTACK_ELEMENT_CORRECT_PARAM_ST, calc_correct_graph::CACL_CORRECT_GRAPH_ST,
        equip_param_weapon::EQUIP_PARAM_WEAPON_ST, reinforce_param_weapon::REINFORCE_PARAM_WEAPON_ST,
        sp_effect::SP_EFFECT_PARAM_ST,
    },
};
use ertypes::stats::{Damage, Effect, Passive, Reinforcement, Stat};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Infusion {
    Standard,
    Heavy,
    Keen,
    Quality,
    Fire,
    FlameArt,
    Lightning,
    Sacred,
    Magic,
    Cold,
    Poison,
    Blood,
    Occult,
}

impl Infusion {
    fn from_id(i: u32) -> anyhow::Result<Self> {
        match i {
            0 => Ok(Self::Standard),
            100 => Ok(Self::Heavy),
            200 => Ok(Self::Keen),
            300 => Ok(Self::Quality),
            400 => Ok(Self::Fire),
            500 => Ok(Self::FlameArt),
            600 => Ok(Self::Lightning),
            700 => Ok(Self::Sacred),
            800 => Ok(Self::Magic),
            900 => Ok(Self::Cold),
            1000 => Ok(Self::Poison),
            1100 => Ok(Self::Blood),
            1200 => Ok(Self::Occult),
            _ => anyhow::bail!("unsupported infusion {i}"),
        }
    }
}

// *_correct_rate

#[derive(Debug)]
pub struct PassiveLvl {
    pub id: u8,
    pub tp: Passive,
    pub base: f32,
}

#[derive(Debug)]
pub struct WeaponInfo {
    pub name: String,
    pub weight: f32,
    pub id: u32,
    pub mainid: u32,
    pub infusion: Infusion,
    pub attack_base: Damage<u16>,
    pub required: Stat<u8>,
    pub correct_a: Stat<f32>,
    pub correct_d: Damage<u8>,
    pub correct_e: Effect<u8>,
    pub reinforce_id: i16,
    pub correct_id: i32,
    pub passives: Vec<PassiveLvl>,
}

fn load_sp_effects(reg: &BND4) -> anyhow::Result<BTreeMap<u32, SP_EFFECT_PARAM_ST>> {
    load_params(reg, "SpEffect", |rid, a: SP_EFFECT_PARAM_ST| (rid, a))
}

fn load_reinforcement(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Reinforcement>> {
    load_params(reg, "ReinforceParamWeapon", |rid, rw: REINFORCE_PARAM_WEAPON_ST| {
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
                sp1: rw.resident_sp_effect_id1,
                sp2: rw.resident_sp_effect_id2,
                sp3: rw.resident_sp_effect_id3,
            },
        )
    })
}

fn load_aec(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Stat<Damage<i16>>>> {
    load_params(
        reg,
        "AttackElementCorrectParam",
        |rid, rw: ATTACK_ELEMENT_CORRECT_PARAM_ST| {
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
    sp_effects: &BTreeMap<u32, SP_EFFECT_PARAM_ST>,
    reinforce: &BTreeMap<u32, Reinforcement>,
) -> anyhow::Result<Vec<WeaponInfo>> {
    let raw_equip_param_weapon = reg.get_file("EquipParamWeapon").unwrap();
    let equip_param_weapon = crate::erformat::params::Params::from_bytes(raw_equip_param_weapon)?;

    let mut weapons = Vec::new();

    for ridx in 0..equip_param_weapon.row_count() {
        let (rid, rdata) = equip_param_weapon.raw_row(ridx);
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
            let wpn = WeaponInfo::new(nm.clone(), rid, &eqpr, sp_effects, reinforce)?;
            weapons.push(wpn);
        }
    }
    Ok(weapons)
}

fn load_correct(reg: &BND4) -> anyhow::Result<BTreeMap<u32, Scaling>> {
    load_params(reg, "CalcCorrectGraph", |rid, clc: CACL_CORRECT_GRAPH_ST| {
        let row = Scaling::new(&clc);
        (rid, row)
    })
}

pub struct WeaponData {
    pub sp_effects: BTreeMap<u32, SP_EFFECT_PARAM_ST>,
    pub reinforcement: BTreeMap<u32, Reinforcement>,
    pub aec: BTreeMap<u32, Stat<Damage<i16>>>,
    pub graphes: BTreeMap<u32, Scaling>,
    pub weapons: Vec<WeaponInfo>,
}

impl WeaponData {
    pub(crate) fn load(reg: &BND4, names: &HashMap<u32, String>) -> anyhow::Result<Self> {
        let sp_effects = load_sp_effects(reg)?;
        let reinforcement = load_reinforcement(reg)?;
        let aec = load_aec(reg)?;
        let graphes = load_correct(reg)?;
        let weapons = load_weapons(reg, names, &sp_effects, &reinforcement)?;
        Ok(Self {
            sp_effects,
            reinforcement,
            aec,
            graphes,
            weapons,
        })
    }
}

impl WeaponInfo {
    pub fn new(
        name: String,
        id: u32,
        eqp: &EQUIP_PARAM_WEAPON_ST,
        sp: &BTreeMap<u32, SP_EFFECT_PARAM_ST>,
        reinforce: &BTreeMap<u32, Reinforcement>,
    ) -> anyhow::Result<Self> {
        let infusion = Infusion::from_id(id % 10000).with_context(|| anyhow::anyhow!("weapon {name}[{id}]"))?;
        let mut passives = Vec::new();

        for (id, effect) in [
            eqp.sp_effect_behavior_id0,
            eqp.sp_effect_behavior_id1,
            eqp.sp_effect_behavior_id2,
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
            let btp = if i.blood_attack_power > 0 {
                Some((i.blood_attack_power, Passive::Blood))
            } else if i.sleep_attack_power > 0 {
                Some((i.sleep_attack_power, Passive::Sleep))
            } else if i.freeze_attack_power > 0 {
                Some((i.freeze_attack_power, Passive::Frost))
            } else if i.disease_attack_power > 0 {
                Some((i.disease_attack_power, Passive::ScarletRot))
            } else if i.madness_attack_power > 0 {
                Some((i.madness_attack_power, Passive::Madness))
            } else if i.poizon_attack_power > 0 {
                Some((i.poizon_attack_power, Passive::Poison))
            } else {
                None
            };
            if let Some((rawbase, tp)) = btp {
                let base = match (eqp.reinforce_type_id, tp, id) {
                    (900, Passive::Frost, _) => (rawbase as f32) * 1.59,
                    (1100, Passive::Poison, 0) | (1100, Passive::Blood, 0) | (1000, Passive::Poison, 1) => {
                        (rawbase as f32) * 1.44
                    }
                    _ => rawbase as f32,
                };
                passives.push(PassiveLvl { id: id as u8, tp, base });
            }
        }
        Ok(Self {
            name,
            weight: eqp.weight,
            id,
            mainid: id / 10000,
            infusion,
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

    pub fn multidamage(&self) -> bool {
        self.attack_base.to_slice().into_iter().filter(|&&x| x != 0).count() > 1
    }
}
