use anyhow::Context;
use serde::Serialize;

use crate::{
    stats::{Damage, Effect, Stat},
    structs::equip_param_weapon::EQUIP_PARAM_WEAPON_ST,
};

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
}

impl WeaponInfo {
    pub fn new(name: String, id: u32, eqp: &EQUIP_PARAM_WEAPON_ST) -> anyhow::Result<Self> {
        let infusion = Infusion::from_id(id % 10000).with_context(|| anyhow::anyhow!("weapon {name}[{id}]"))?;
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
            },
            reinforce_id: eqp.reinforce_type_id,
            correct_id: eqp.attack_element_correct_id,
        })
    }
}

pub fn max_scaling(reinforce_id: i16) -> Stat<f32> {
    match reinforce_id {
        0 => Stat {
            str: 1.5,
            dex: 1.5,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // default
        100 => Stat {
            str: 2.8,
            dex: 0.0,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // heavy high
        200 => Stat {
            str: 1.3,
            dex: 2.8,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // keen high
        300 => Stat {
            str: 1.9,
            dex: 1.9,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // quality
        400 => Stat {
            str: 2.1,
            dex: 1.2,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // fire
        500 => Stat {
            str: 1.8,
            dex: 1.8,
            int: 1.8,
            fth: 2.3,
            arc: 1.8,
        }, // flame art
        600 => Stat {
            str: 1.2,
            dex: 2.1,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // lightning
        700 => Stat {
            str: 1.8,
            dex: 1.8,
            int: 1.8,
            fth: 2.3,
            arc: 1.8,
        }, // sacred
        800 => Stat {
            str: 1.3,
            dex: 1.3,
            int: 2.35,
            fth: 1.8,
            arc: 1.8,
        }, // magic
        900 => Stat {
            str: 1.9,
            dex: 1.9,
            int: 2.0,
            fth: 1.8,
            arc: 1.8,
        }, // cold
        1000 | 1100 => Stat {
            str: 1.9,
            dex: 1.9,
            int: 1.9,
            fth: 1.9,
            arc: 1.45,
        }, // poison / blood
        1200 => Stat {
            str: 1.5,
            dex: 1.5,
            int: 1.5,
            fth: 1.5,
            arc: 1.8,
        }, // occult
        1300 => Stat {
            str: 1.5,
            dex: 1.5,
            int: 4.3,
            fth: 2.4,
            arc: 2.4,
        }, // carian sorcery sword
        3000 => Stat {
            str: 1.0,
            dex: 1.0,
            int: 1.0,
            fth: 1.0,
            arc: 1.0,
        }, // no reinforce
        5000 => Stat {
            str: 1.3,
            dex: 2.5,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // keen low
        6000 => Stat {
            str: 2.6,
            dex: 1.2,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // heavy low
        2200 => Stat::all(1.8),               // somber
        1900 | 2400 => Stat::all(3.0),        // staves
        3100 | 3200 | 3300 => Stat::all(0.0), // crossbows
        8000 | 8100 | 8200 => Stat {
            str: 1.8,
            dex: 1.8,
            int: 1.8,
            fth: 1.8,
            arc: 1.8,
        }, // shields
        8300 | 8500 => Stat::all(3.0),        // shields 4 / coil shield

        _ => panic!("unknown reinforce_id: {reinforce_id}"),
    }
}
