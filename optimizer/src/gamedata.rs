use std::{collections::HashMap, path::Path};

use eldenring_companion::{Absorptions, Armor, ArmorCategory, Body, Resistances};
use serde_json::Value;

pub struct GameData {
    pub armors: Body<Vec<Armor>>,
}

#[derive(Debug)]
pub enum Game {
    EldenRing,
    DS3,
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ds3" => Ok(Game::DS3),
            "elden ring" | "eldenring" | "elden_ring" => Ok(Game::EldenRing),
            _ => Err(format!("Can't recognize game {}", s)),
        }
    }
}

pub(super) fn load_data(dir: &Path, game: Game) -> GameData {
    match game {
        Game::DS3 => ds3(dir),
        Game::EldenRing => elden_ring(dir),
    }
}

fn elden_ring(dir: &Path) -> GameData {
    let mut p = dir.to_path_buf();
    p.push("armor.json");
    let f_armor = std::fs::File::open(p).unwrap();
    let raw_armors: HashMap<String, Armor> = serde_json::from_reader(f_armor).unwrap();
    gamedata(raw_armors.into_values())
}

fn gamedata<I: Iterator<Item = Armor>>(raw_armors: I) -> GameData {
    let mut armors: Body<Vec<Armor>> = Body::default();
    for r in raw_armors {
        match &r.category {
            ArmorCategory::Head => armors.head.push(r),
            ArmorCategory::Arms => armors.arms.push(r),
            ArmorCategory::Body => armors.body.push(r),
            ArmorCategory::Legs => armors.legs.push(r),
        }
    }
    GameData { armors }
}

fn ds3(dir: &Path) -> GameData {
    let mut p = dir.to_path_buf();
    p.push("armor.json");
    let f_armor = std::fs::File::open(p).unwrap();

    let raw_armors: Vec<Vec<Value>> = serde_json::from_reader(f_armor).unwrap();
    fn decode_values(v: Vec<Value>) -> Armor {
        if v.len() < 18 || v.len() > 19 {
            panic!("invalid row {:?}", v);
        }
        let name = v[1].as_str().unwrap().to_string();
        let weight = v[2].as_f64().unwrap();
        let physical = v[3].as_f64().unwrap();
        let strike = v[4].as_f64().unwrap();
        let slash = v[5].as_f64().unwrap();
        let pierce = v[6].as_f64().unwrap();
        let magic = v[7].as_f64().unwrap();
        let fire = v[8].as_f64().unwrap();
        let lightning = v[9].as_f64().unwrap();
        let holy = v[10].as_f64().unwrap(); // dark
        let focus = v[11].as_f64().unwrap(); // bleed
        let immunity = v[12].as_f64().unwrap();
        let robustness = v[13].as_f64().unwrap(); // frost
        let vitality = v[14].as_f64().unwrap();
        let poise = v[15].as_f64().unwrap();
        let category = match v[16].as_str().unwrap() {
            "Body" => ArmorCategory::Body,
            "Arms" => ArmorCategory::Arms,
            "Legs" => ArmorCategory::Legs,
            "Head" => ArmorCategory::Head,
            x => panic!("unknown category {}", x),
        };

        Armor {
            category,
            name,
            weight,
            absorptions: Absorptions {
                fire,
                holy,
                lightning,
                magic,
                physical,
                pierce,
                slash,
                strike,
            },
            resistances: Resistances {
                focus,
                immunity,
                poise,
                robustness,
                vitality,
            },
        }
    }
    gamedata(raw_armors.into_iter().map(decode_values))
}
