use std::collections::{BTreeMap, HashMap};

use serde::Serialize;

use crate::{
    scaling::Scaling,
    stats::{Damage, Stat},
    weaponinfo::{max_scaling, WeaponInfo},
};

#[derive(Debug, Serialize)]
pub struct Best {
    pub score: f32,
    pub stats: Stat<u8>,
    pub damage: Damage<u32>,
    // pub effect: Effect<u32>,
}

pub(crate) fn best_stats(
    wpn: &WeaponInfo,
    reinforce: &BTreeMap<u32, Damage<f32>>,
    graphes: &HashMap<u32, Scaling>,
    accorrect: &BTreeMap<u32, Stat<Damage<i16>>>,
    mixed_damage_scale: f32,
    two_handed: bool,
) -> BTreeMap<u32, Best> {
    let statbounds = wpn.required.map2_r(&wpn.correct_a, |&req, &cor| {
        if cor > 0.0 {
            (std::cmp::max(req, 10), 100)
        } else {
            (10, 11)
        }
    });

    let damages = wpn
        .correct_d
        .fmap_r(|&dmg_type| graphes.get(&(dmg_type as u32)).unwrap());

    let reinforce_scale = reinforce.get(&(wpn.reinforce_id as u32)).unwrap();
    let accorrect = accorrect.get(&(wpn.correct_id as u32)).unwrap();

    let mut out = BTreeMap::new();

    for str in statbounds.str.0..statbounds.str.1 {
        for dex in statbounds.dex.0..statbounds.dex.1 {
            for int in statbounds.int.0..statbounds.int.1 {
                for fth in statbounds.fth.0..statbounds.fth.1 {
                    for arc in statbounds.arc.0..statbounds.arc.1 {
                        let level = str as u32 + dex as u32 + int as u32 + fth as u32 + arc as u32 - 50;
                        let curstats = Stat {
                            str: if two_handed { str + (str / 2) } else { str },
                            dex,
                            int,
                            fth,
                            arc,
                        };
                        let saturation = curstats.fmap(|stt| damages.fmap_r(|sc| sc.power(stt)));
                        let base_damage = wpn.attack_base.map2_r(reinforce_scale, |&a, &b| a as f32 * b);
                        let base_scaling = wpn
                            .correct_a
                            .map2_r(&max_scaling(wpn.reinforce_id), |&a, &b| (a * b / 100.0));

                        let scaled_sat = saturation
                            .map2(base_scaling, |a, b| a.fmap_r(|&x| x * b))
                            .map2_r(accorrect, |a, b| a.map2_r(b, |&x, &y| x * (y as f32) / 100.0));

                        let sat_damages = scaled_sat
                            .str
                            .map2(scaled_sat.dex, |a, b| a + b)
                            .map2(scaled_sat.int, |a, b| a + b)
                            .map2(scaled_sat.fth, |a, b| a + b)
                            .map2(scaled_sat.arc, |a, b| a + b);

                        let damages = base_damage.map2(sat_damages, |base, st| base * (1.0 + st / 100.0));
                        let mut all_damages: [f32; 5] = [
                            damages.physics,
                            damages.fire,
                            damages.holy,
                            damages.lightning,
                            damages.magic,
                        ];
                        all_damages.sort_by(|&a, &b| b.total_cmp(&a));
                        let score: f32 =
                            all_damages[0] + all_damages[1..].iter().copied().sum::<f32>() * mixed_damage_scale;

                        let cur = Best {
                            score,
                            stats: Stat {
                                str,
                                dex,
                                int,
                                fth,
                                arc,
                            },
                            damage: damages.fmap_r(|&x| x as u32),
                        };

                        match out.get(&level) {
                            None => {
                                out.insert(level, cur);
                            }
                            Some(cursol) => {
                                if cursol.score < score {
                                    out.insert(level, cur);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    out
}
