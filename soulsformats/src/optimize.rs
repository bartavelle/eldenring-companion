use serde::Serialize;
use std::collections::BTreeMap;

use crate::{
    scaling::Scaling,
    weaponinfo::{WeaponData, WeaponInfo},
};
use ertypes::stats::{Damage, Effect, Passive, Stat};

#[derive(Debug, Serialize)]
pub struct Best {
    pub score: f32,
    pub stats: Stat<u8>,
    pub damage: Damage<u32>,
    pub effect: Vec<(Passive, f32)>,
    // pub effect: Effect<u32>,
}

#[derive(Debug, Default)]
pub struct Scaled<A> {
    pub r100: A,
    pub r75: A,
    pub r50: A,
}

pub struct WeaponStats<'t> {
    pub base_damage: Damage<f32>,
    pub base_scaling: Stat<f32>,
    pub base_damage_scaler: Stat<Damage<f32>>,
    pub constant_effect: Option<Vec<(Passive, f32)>>,
    pub damages: Damage<&'t Scaling>,
    pub effects: Effect<&'t Scaling>,
}

const DEFAULT_CORRECT: Stat<Damage<i16>> = Stat {
    str: Damage {
        physics: 100,
        magic: 0,
        fire: 0,
        lightning: 0,
        holy: 0,
    },
    dex: Damage {
        physics: 100,
        magic: 0,
        fire: 0,
        lightning: 0,
        holy: 0,
    },
    int: Damage {
        physics: 0,
        magic: 100,
        fire: 0,
        lightning: 0,
        holy: 0,
    },
    fth: Damage {
        physics: 0,
        magic: 100,
        fire: 0,
        lightning: 0,
        holy: 0,
    },
    arc: Damage {
        physics: 0,
        magic: 0,
        fire: 0,
        lightning: 0,
        holy: 0,
    },
};

impl<'t> WeaponStats<'t> {
    pub fn new(wpn: &WeaponInfo, wpndata: &'t WeaponData) -> Self {
        let damages = wpn
            .correct_d
            .fmap_r(|&dmg_type| wpndata.graphes.get(&(dmg_type as u32)).unwrap());

        let effects = wpn
            .correct_e
            .fmap_r(|&dmg_type| wpndata.graphes.get(&(dmg_type as u32)).unwrap());

        let reinforcement = wpndata.reinforcement.get(&(wpn.reinforce_id as u32)).unwrap();
        let accorrect = if wpndata.aec.is_empty() {
            &DEFAULT_CORRECT
        } else {
            wpndata.aec.get(&(wpn.correct_id as u32)).unwrap()
        };

        let constant_effect = if wpn
            .passives
            .iter()
            .any(|p| p.tp == Passive::Blood || p.tp == Passive::Poison)
        {
            None
        } else {
            Some(wpn.passives.iter().map(|psv| (psv.tp, psv.base)).collect::<Vec<_>>())
        };
        let base_damage = wpn.attack_base.map2_r(&reinforcement.damage, |&a, &b| a as f32 * b);
        let base_scaling = wpn.correct_a.map2_r(&reinforcement.stats, |&a, &b| (a * b / 100.0));

        let base_damage_scaler = accorrect.map2_r(&base_scaling, |&ac, &y| ac.fmap_r(|&x| (x as f32) * y / 100.0));

        WeaponStats {
            constant_effect,
            base_damage,
            base_scaling,
            base_damage_scaler,
            damages,
            effects,
        }
    }
}

pub fn calc_damage(wpn: &WeaponInfo, wpndata: &WeaponData, stats: Stat<u8>, mixlevel: f32) -> Best {
    let WeaponStats {
        base_damage,
        base_scaling,
        base_damage_scaler,
        constant_effect,
        damages,
        effects,
    } = WeaponStats::new(wpn, wpndata);
    let effect = constant_effect.clone().unwrap_or_else(|| {
        let saturation_e = effects.fmap_r(|sc| sc.power(stats.arc) * base_scaling.arc / 100.0);

        wpn.passives
            .iter()
            .map(|psv| {
                (
                    psv.tp,
                    psv.base
                        * (1.0
                            + match psv.tp {
                                Passive::Blood => saturation_e.blood,
                                Passive::Poison => saturation_e.poison,
                                _ => 0.0,
                            }),
                )
            })
            .collect()
    });
    let arc_damage = damages.map2_r(&base_damage_scaler.arc, |sc, &y| sc.power(stats.arc) * y);
    let dex_damage = damages.map2_r(&base_damage_scaler.dex, |sc, &y| sc.power(stats.dex) * y);
    let str_damage = damages.map2_r(&base_damage_scaler.str, |sc, &y| sc.power(stats.str) * y);
    let int_damage = damages.map2_r(&base_damage_scaler.int, |sc, &y| sc.power(stats.int) * y);
    let fth_damage = damages.map2_r(&base_damage_scaler.fth, |sc, &y| sc.power(stats.fth) * y);
    let sat_damages = arc_damage
        .map2(dex_damage, |a, b| a + b)
        .map2(int_damage, |a, b| a + b)
        .map2(str_damage, |a, b| a + b)
        .map2(fth_damage, |a, b| a + b);

    let damages = base_damage.map2(sat_damages, |base, st| base * (1.0 + st / 100.0));
    let mut all_damages: [f32; 5] = [
        damages.physics,
        damages.fire,
        damages.holy,
        damages.lightning,
        damages.magic,
    ];
    all_damages.sort_by(|&a, &b| b.total_cmp(&a));
    let score0: f32 = all_damages[0];
    let scoren = all_damages[1..].iter().copied().sum::<f32>();
    Best {
        score: score0 + scoren * mixlevel,
        stats,
        damage: damages.fmap_r(|&x| x as u32),
        effect,
    }
}

pub fn best_stats(
    wpn: &WeaponInfo,
    wpndata: &WeaponData,
    two_handed: bool,
    mins: Stat<u8>,
) -> Scaled<BTreeMap<u32, Best>> {
    let statbounds = wpn
        .required
        .map2_r(&wpn.correct_a, |&req, &cor| {
            if cor > 0.0 {
                (std::cmp::max(req, 10), 100)
            } else {
                (req, req + 1)
            }
        })
        .map2(mins, |(l, h), mn| (std::cmp::max(l, mn), std::cmp::max(h, mn + 1)));
    let WeaponStats {
        base_damage,
        base_scaling,
        base_damage_scaler,
        constant_effect,
        damages,
        effects,
    } = WeaponStats::new(wpn, wpndata);

    let mut out = Scaled::default();

    for arc in statbounds.arc.0..statbounds.arc.1 {
        let arc_damage = damages.map2_r(&base_damage_scaler.arc, |sc, &y| sc.power(arc) * y);
        let curdamage = arc_damage;
        let effect = constant_effect.clone().unwrap_or_else(|| {
            let saturation_e = effects.fmap_r(|sc| sc.power(arc) * base_scaling.arc / 100.0);

            wpn.passives
                .iter()
                .map(|psv| {
                    (
                        psv.tp,
                        psv.base
                            * (1.0
                                + match psv.tp {
                                    Passive::Blood => saturation_e.blood,
                                    Passive::Poison => saturation_e.poison,
                                    _ => 0.0,
                                }),
                    )
                })
                .collect()
        });

        for dex in statbounds.dex.0..statbounds.dex.1 {
            // these tests can cut computation times to 1/3rd. They are written in this way because Stat only provides a partial order
            if arc > statbounds.arc.0 && base_damage_scaler.dex > base_damage_scaler.arc && dex < arc {
                continue;
            }
            if dex > statbounds.dex.0 && base_damage_scaler.arc > base_damage_scaler.dex && arc < dex {
                continue;
            }
            let dex_damage = damages.map2_r(&base_damage_scaler.dex, |sc, &y| sc.power(dex) * y);
            let curdamage = curdamage.map2(dex_damage, |a, b| a + b);
            for int in statbounds.int.0..statbounds.int.1 {
                if arc > statbounds.arc.0 && base_damage_scaler.int > base_damage_scaler.arc && int < arc {
                    continue;
                }
                if int > statbounds.int.0 && base_damage_scaler.arc > base_damage_scaler.int && arc < int {
                    continue;
                }
                if dex > statbounds.dex.0 && base_damage_scaler.int > base_damage_scaler.dex && int < dex {
                    continue;
                }
                if int > statbounds.int.0 && base_damage_scaler.dex > base_damage_scaler.int && dex < int {
                    continue;
                }

                let int_damage = damages.map2_r(&base_damage_scaler.int, |sc, &y| sc.power(int) * y);
                let curdamage = curdamage.map2(int_damage, |a, b| a + b);
                for fth in statbounds.fth.0..statbounds.fth.1 {
                    if arc > statbounds.arc.0 && base_damage_scaler.fth > base_damage_scaler.arc && fth < arc {
                        continue;
                    }
                    if fth > statbounds.fth.0 && base_damage_scaler.arc > base_damage_scaler.fth && arc < fth {
                        continue;
                    }
                    if dex > statbounds.dex.0 && base_damage_scaler.fth > base_damage_scaler.dex && fth < dex {
                        continue;
                    }
                    if fth > statbounds.fth.0 && base_damage_scaler.dex > base_damage_scaler.fth && dex < fth {
                        continue;
                    }
                    if int > statbounds.int.0 && base_damage_scaler.fth > base_damage_scaler.int && fth < int {
                        continue;
                    }
                    if fth > statbounds.fth.0 && base_damage_scaler.int > base_damage_scaler.fth && int < fth {
                        continue;
                    }

                    let fth_damage = damages.map2_r(&base_damage_scaler.fth, |sc, &y| sc.power(fth) * y);
                    let curdamage = curdamage.map2(fth_damage, |a, b| a + b);
                    for base_str in statbounds.str.0..statbounds.str.1 {
                        let str = if two_handed {
                            base_str + (base_str / 2)
                        } else {
                            base_str
                        };
                        let str_damage = damages.map2_r(&base_damage_scaler.str, |sc, &y| sc.power(str) * y);
                        let curdamage = curdamage.map2(str_damage, |a, b| a + b);

                        let level = str as u32 + dex as u32 + int as u32 + fth as u32 + arc as u32 - 50;
                        let sat_damages = curdamage;
                        let damages = base_damage.map2(sat_damages, |base, st| base * (1.0 + st / 100.0));
                        let mut all_damages: [f32; 5] = [
                            damages.physics,
                            damages.fire,
                            damages.holy,
                            damages.lightning,
                            damages.magic,
                        ];
                        all_damages.sort_by(|&a, &b| b.total_cmp(&a));
                        let score0: f32 = all_damages[0];
                        let scoren = all_damages[1..].iter().copied().sum::<f32>();

                        let score100 = score0 + scoren;
                        let score75 = score0 + scoren * 0.75;
                        let score50 = score0 + scoren * 0.5;
                        let damage = damages.fmap_r(|&x| x as u32);

                        let insert_score = |mp: &mut BTreeMap<u32, Best>, score, effect| {
                            let cur = Best {
                                score,
                                stats: Stat {
                                    str: base_str,
                                    dex,
                                    int,
                                    fth,
                                    arc,
                                },
                                damage,
                                effect,
                            };

                            match mp.get(&level) {
                                None => {
                                    mp.insert(level, cur);
                                }
                                Some(cursol) => {
                                    if cursol.score < score {
                                        mp.insert(level, cur);
                                    }
                                }
                            }
                        };

                        insert_score(&mut out.r100, score100, effect.clone());
                        insert_score(&mut out.r75, score75, effect.clone());
                        insert_score(&mut out.r50, score50, effect.clone());
                    }
                }
            }
        }
    }

    out
}
