use clap::{Parser, Subcommand};
use ertypes::stats::Stat;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::RwLock,
};
use weaponinfo::{Infusion, WeaponData, WeaponInfo};

pub mod armor;
pub mod erformat;
pub mod optimize;
pub mod regulation;
pub mod scaling;
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
    ArmorDump,
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

    let weapon_names = load_names(&args.data, "EquipParamWeapon.txt")?;
    let regulations = regulation::load_regulation(&args.regulation)?;
    eprintln!("regulation version {}", regulations.version);

    match args.command {
        Command::ArmorDump => {
            let armor_names = load_names(&args.data, "EquipParamProtector.txt")?;
            let armor = armor::load_armor(&regulations, &armor_names)?;
            // println!("{armor:?}");
        }
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
            let wpn_data = WeaponData::load(&regulations, &weapon_names)?;
            let mins = Stat {
                str: min_str,
                dex: min_dex,
                int: min_int,
                fth: min_fth,
                arc: min_arc,
            };
            let wpn = wpn_data
                .weapons
                .iter()
                .find(|w| w.name == weapon)
                .expect("could not find weapon");
            let best = optimize::best_stats(wpn, &wpn_data, two_handed, mins);
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
            let wpn_data = WeaponData::load(&regulations, &weapon_names)?;
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
            for wpn in &wpn_data.weapons {
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
                let optim_result =
                    optimize::best_stats(wpn, &wpn_data, params.handling == THStatus::TwoHands, Stat::all(10));

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
