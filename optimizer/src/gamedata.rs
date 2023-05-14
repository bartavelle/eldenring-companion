use std::{collections::HashMap, path::Path};

use eldenring_companion::{Armor, ArmorCategory, Body};

pub struct GameData {
    pub armors: Body<Vec<Armor>>,
}

pub(super) fn load_data(dir: &Path) -> GameData {
    let mut p = dir.to_path_buf();
    p.push("armor.json");
    let f_armor = std::fs::File::open(p).unwrap();
    let raw_armors: HashMap<String, Armor> = serde_json::from_reader(f_armor).unwrap();
    let mut armors: Body<Vec<Armor>> = Body::default();
    for (_, r) in raw_armors {
        match &r.category {
            ArmorCategory::Head => armors.head.push(r),
            ArmorCategory::Arms => armors.arms.push(r),
            ArmorCategory::Body => armors.body.push(r),
            ArmorCategory::Legs => armors.legs.push(r),
        }
    }
    GameData { armors }
}
