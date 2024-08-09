use std::collections::{BTreeMap, HashMap};

use ertypes::stats::{Damage, Effect, Passive, Reinforcement, Stat};
use serde::Serialize;

use crate::{formats::bnd4::BND4, scaling::Scaling};

mod ds1r;
mod er;

#[derive(Debug, Serialize)]
pub struct PassiveLvl {
    pub id: u8,
    pub tp: Passive,
    pub base: f32,
}

#[derive(Debug, Serialize)]
pub struct WeaponInfo {
    pub name: String,
    pub weight: f32,
    pub id: u32,
    pub mainid: u32,
    pub attack_base: Damage<u16>,
    pub required: Stat<u8>,
    pub correct_a: Stat<f32>,
    pub correct_d: Damage<u8>,
    pub correct_e: Effect<u8>,
    pub reinforce_id: i16,
    pub correct_id: i32,
    pub passives: Vec<PassiveLvl>,
}

#[derive(Serialize)]
pub struct SpEffect {
    poizon_attack_power: f32,
    disease_attack_power: f32,
    blood_attack_power: f32,
    freeze_attack_power: f32,
    sleep_attack_power: f32,
    madness_attack_power: f32,
    name: Option<String>,
}

#[derive(Serialize)]
pub struct WeaponData {
    pub sp_effects: BTreeMap<u32, SpEffect>,
    pub reinforcement: BTreeMap<u32, Reinforcement>,
    pub aec: BTreeMap<u32, Stat<Damage<i16>>>,
    pub graphes: BTreeMap<u32, Scaling>,
    pub weapons: Vec<WeaponInfo>,
}

impl WeaponInfo {
    pub fn multidamage(&self) -> bool {
        self.attack_base.to_slice().into_iter().filter(|&&x| x != 0).count() > 1
    }
}

impl WeaponData {
    pub fn load_er(reg: &BND4, names: &HashMap<u32, String>) -> anyhow::Result<Self> {
        er::load_weapondata(reg, names)
    }

    pub fn load_ds1r(reg: &BND4, weapon_names: &HashMap<u32, String>) -> anyhow::Result<Self> {
        ds1r::load_weapondata(reg, weapon_names)
    }
}
