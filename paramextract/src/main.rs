use clap::{Parser, Subcommand};
use packed_struct::PackedStructSlice;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use scaling::Scaling;
use serde::Serialize;
use stats::{Damage, Reinforcement, Stat};
use std::{
    collections::{BTreeMap, HashMap},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::RwLock,
};
use weaponinfo::{Infusion, WeaponInfo};

pub mod bnd4;
pub mod optimize;
pub mod params;
pub mod scaling;
pub mod stats;
pub mod structs;
pub mod utils;
pub mod weaponinfo;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the regulations file")]
    regulation: PathBuf,
    #[arg(short, long, help = "Data dir (contains names)")]
    data: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Optimize {
        #[arg(short, long, help = "Weapon to optimize for")]
        weapon: String,
        #[arg(short, long, help = "Mixed damage penalty (for lower damages)")]
        mixed_damage_scale: Option<f32>,
        #[arg(short, long, help = "should the weapon be two handed")]
        two_handed: bool,
        #[arg(long, default_value_t = 10)]
        min_str: u8,
        #[arg(long, default_value_t = 10)]
        min_dex: u8,
        #[arg(long, default_value_t = 10)]
        min_int: u8,
        #[arg(long, default_value_t = 10)]
        min_fth: u8,
        #[arg(long, default_value_t = 10)]
        min_arc: u8,
    },
    GenAll {
        #[arg(short, long, help = "Output directory")]
        out: PathBuf,
        #[arg(short, long, help = "Limit amount of computer json (for testing)")]
        limit: Option<usize>,
    },
}

static REGULATION_KEY: [u8; 32] = [
    0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02, 0xD6, 0x76, 0xC4, 0x28, 0x92,
    0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E, 0x49, 0x3F, 0xEF, 0x99,
];

fn decrypt(key: &[u8; 32], encrypted: &[u8]) -> Vec<u8> {
    let iv = &encrypted[..16];
    let data = &encrypted[16..];
    let cipher = libaes::Cipher::new_256(key);
    cipher.cbc_decrypt(iv, data)
}

fn load_names(dir: &Path, name: &str) -> anyhow::Result<HashMap<u32, String>> {
    let mut dir = dir.to_owned();
    dir.push(name);
    let f = std::fs::File::open(dir)?;
    let b = BufReader::new(f);
    let mut out = HashMap::new();
    for l in b.lines() {
        let l = l?;
        match l.split_once(' ') {
            Some((rawid, nm)) => {
                let id = rawid.parse()?;
                out.insert(id, nm.to_string());
            }
            None => panic!("malformed name line {l}"),
        }
    }
    Ok(out)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // let dec_prm = std::fs::read(args.parambnd)?;
    // let raw_prm = bnd4::decompress(dec_prm);
    // let parambnd = bnd4::BND4::parse(raw_prm).unwrap();

    let weapons_names = load_names(&args.data, "EquipParamWeapon.txt")?;

    let enc_reg = std::fs::read(args.regulation)?;
    let dec_reg = decrypt(&REGULATION_KEY, &enc_reg);
    let raw_reg = bnd4::decompress(dec_reg);
    let regulations = bnd4::BND4::parse(raw_reg).unwrap();
    eprintln!("regulation version {}", regulations.version);

    let raw_spe = regulations.get_file("SpEffect").unwrap();
    let spe = params::Params::from_bytes(raw_spe)?;
    let mut sp_effects = BTreeMap::new();
    for ridx in 0..spe.row_count() {
        let (rid, rdata) = spe.raw_row(ridx);
        let rw = structs::sp_effect::SP_EFFECT_PARAM_ST::unpack_from_slice(rdata)?;
        sp_effects.insert(rid, rw);
    }

    let mut reinforce = BTreeMap::new();
    let raw_rein = regulations.get_file("ReinforceParamWeapon").unwrap();
    let rein = params::Params::from_bytes(raw_rein)?;
    // they are stored in increasing order, so we can just update
    for ridx in 0..rein.row_count() {
        let (rid, rdata) = rein.raw_row(ridx);
        let rw = structs::reinforce_param_weapon::REINFORCE_PARAM_WEAPON_ST::unpack_from_slice(rdata)?;
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
        reinforce.insert(
            reinforce_type,
            Reinforcement {
                damage: reinforce_damage,
                stats: reinforce_stats,
                sp1: rw.resident_sp_effect_id1,
                sp2: rw.resident_sp_effect_id2,
                sp3: rw.resident_sp_effect_id3,
            },
        );
    }

    let raw_aec = regulations.get_file("AttackElementCorrectParam").unwrap();
    let aec = params::Params::from_bytes(raw_aec)?;
    // they are stored in increasing order, so we can just update
    let mut attack_correct_param = BTreeMap::new();
    for ridx in 0..aec.row_count() {
        let (rid, rdata) = aec.raw_row(ridx);
        let rw = structs::attack_element_correct::ATTACK_ELEMENT_CORRECT_PARAM_ST::unpack_from_slice(rdata)?;
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
        attack_correct_param.insert(rid, correct);
    }

    let raw_equip_param_weapon = regulations.get_file("EquipParamWeapon").unwrap();
    let equip_param_weapon = params::Params::from_bytes(raw_equip_param_weapon)?;

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
        let eqpr = structs::equip_param_weapon::EQUIP_PARAM_WEAPON_ST::unpack_from_slice(rdata)?;
        if let Some(nm) = weapons_names.get(&rid) {
            let wpn = WeaponInfo::new(nm.clone(), rid, &eqpr, &sp_effects, &reinforce)?;
            weapons.push(wpn);
        }
    }

    let raw_calc_correct_graph = regulations.get_file("CalcCorrectGraph").unwrap();
    let calc_correct_graph = params::Params::from_bytes(raw_calc_correct_graph)?;
    let mut graphes = HashMap::new();
    for ridx in 0..calc_correct_graph.row_count() {
        let (rid, rdata) = calc_correct_graph.raw_row(ridx);
        let clc = structs::calc_correct_graph::CACL_CORRECT_GRAPH_ST::unpack_from_slice(rdata)?;
        let row = Scaling::new(&clc);
        graphes.insert(rid, row);
    }

    match args.command {
        Command::Optimize {
            weapon,
            mixed_damage_scale,
            two_handed,
            min_str,
            min_dex,
            min_int,
            min_fth,
            min_arc,
        } => {
            let mins = Stat {
                str: min_str,
                dex: min_dex,
                int: min_int,
                fth: min_fth,
                arc: min_arc,
            };
            let wpn = weapons
                .iter()
                .find(|w| w.name == weapon)
                .expect("could not find weapon");
            let best = optimize::best_stats(wpn, &reinforce, &graphes, &attack_correct_param, two_handed, mins);
            let rbest = match mixed_damage_scale {
                None => best.r75,
                Some(1.0) => best.r100,
                Some(0.75) => best.r75,
                Some(0.5) => best.r50,
                _ => panic!("only handles level mixed level 1 0.75 or 0.5"),
            };
            for r in rbest {
                println!("{r:?}")
            }
        }
        Command::GenAll { out, limit } => {
            #[derive(PartialEq, Eq, Serialize, Debug, Clone, Copy)]
            enum THStatus {
                OneHand,
                TwoHands,
                Any,
            }
            #[derive(Serialize, Clone, Debug)]
            struct WpnParams<'t> {
                mixed: f32,
                handling: THStatus,
                id: u32,
                name: &'t str,
                mainid: u32,
                infusion: Infusion,
            }
            let mut todo = Vec::new();
            // generate all json files in /tmp/machin
            for wpn in &weapons {
                // multiple or single damage type
                if wpn.correct_a.str > 0.0 {
                    for th in [THStatus::OneHand, THStatus::TwoHands] {
                        todo.push((
                            WpnParams {
                                mixed: 1.0,
                                handling: th,
                                id: wpn.id,
                                name: &wpn.name,
                                mainid: wpn.mainid,
                                infusion: wpn.infusion,
                            },
                            wpn,
                        ));
                    }
                } else {
                    todo.push((
                        WpnParams {
                            mixed: 1.0,
                            handling: THStatus::Any,
                            id: wpn.id,
                            name: &wpn.name,
                            mainid: wpn.mainid,
                            infusion: wpn.infusion,
                        },
                        wpn,
                    ));
                }
            }

            if let Some(limit) = limit {
                // TODO: properly resize
                todo = todo.into_iter().take(limit).collect();
            }

            eprintln!("TODO: {} elements", todo.len());

            let index: RwLock<Vec<WpnParams>> = RwLock::new(Vec::new());

            todo.par_iter().for_each(|(params, wpn)| {
                let start = std::time::Instant::now();
                let thi = match params.handling {
                    THStatus::Any => "NI",
                    THStatus::OneHand => "1H",
                    THStatus::TwoHands => "2H",
                };
                let optim_result = optimize::best_stats(
                    wpn,
                    &reinforce,
                    &graphes,
                    &attack_correct_param,
                    params.handling == THStatus::TwoHands,
                    Stat::all(10),
                );

                let save_file = |mixed: f32, wpn: &WeaponInfo, best| {
                    let mut path = out.clone();
                    path.push(format!("{}-{thi}-{}.json", wpn.id, mixed));
                    let mut fo = std::fs::File::create(path).unwrap();
                    serde_json::to_writer(&mut fo, &best).unwrap();
                };

                let mut lk = index.write().unwrap();

                if wpn.multidamage() {
                    let mut p75 = params.clone();
                    p75.mixed = 0.75;
                    save_file(0.75, wpn, optim_result.r75);
                    lk.push(p75);

                    let mut p50 = params.clone();
                    p50.mixed = 0.5;
                    save_file(0.5, wpn, optim_result.r50);
                    lk.push(p50);

                    save_file(1.0, wpn, optim_result.r100);
                    lk.push(params.clone());
                } else {
                    save_file(1.0, wpn, optim_result.r100);
                    lk.push(params.clone())
                }

                let end = std::time::Instant::now();
                let elapsed = end - start;
                if elapsed > std::time::Duration::from_secs(5) {
                    println!("{:?} {:.2}s", params, elapsed.as_secs_f32())
                }
            });

            // write summary
            let params = index.into_inner().unwrap();
            let mut path = out.clone();

            path.push("index.json");
            let mut fo = std::fs::File::create(path).unwrap();
            serde_json::to_writer(&mut fo, &params).unwrap();
        }
    }

    Ok(())
}
