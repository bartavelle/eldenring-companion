use clap::{Parser, Subcommand, ValueEnum};
use ertypes::stats::{Stat, Statistic};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use soulsformats::optimize::calc_damage;
use soulsformats::weaponinfo::{WeaponData, WeaponInfo};
use std::{
    collections::{BTreeMap, HashMap},
    io::{stdout, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, RwLock},
};

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

#[derive(Debug, Clone, Copy)]
struct StatValue {
    stat: Statistic,
    value: u8,
}

fn parse_statvalue(s: &str) -> Result<StatValue, String> {
    match s.split_once(':').or_else(|| s.split_once('=')) {
        None => Err("must be formed like STAT=LEVEL, ie. str=80".to_string()),
        Some((l, r)) => {
            let stat = Statistic::from_str(l, true)?;
            let value = r.parse::<u8>().map_err(|rr| rr.to_string())?;
            Ok(StatValue { stat, value })
        }
    }
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
    WeaponDump,
    WeaponSearch {
        #[arg(value_parser = parse_statvalue, help = "Main stats, in order")]
        stats: Vec<StatValue>,
        #[arg(short, long, help = "Mixed damage penalty (for lower damages)")]
        mixed_damage_scale: Option<f32>,
        #[arg(short, long, help = "Amount of weapons to display", default_value_t = 30)]
        limit: usize,
    },
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

    let weapon_names = load_names(&args.data, "EquipParamWeapon.txt")?;
    let regulations = soulsformats::regulation::load_regulation(&args.regulation)?;
    eprintln!("regulation version {}", regulations.version);

    match args.command {
        Command::ArmorDump => {
            let armor_names = load_names(&args.data, "EquipParamProtector.txt")?;
            let armor = soulsformats::armor::load_armor(&regulations, &armor_names)?;
            let to_json = armor
                .into_values()
                .map(|armor| (armor.name.clone(), armor))
                .collect::<BTreeMap<_, _>>();
            serde_json::to_writer_pretty(stdout(), &to_json)?;
        }
        Command::WeaponDump => {
            let wpn_data = WeaponData::load_er(&regulations, &weapon_names)?;
            serde_json::to_writer_pretty(stdout(), &wpn_data)?;
        }
        Command::WeaponSearch {
            stats,
            mixed_damage_scale,
            limit,
        } => {
            let wpn_data = WeaponData::load_er(&regulations, &weapon_names)?;
            let mut rstats = Stat::all(10);
            for stt in &stats {
                rstats.set(stt.stat, stt.value);
            }
            // filter weapons that matches the stats
            let mut wpns = wpn_data
                .weapons
                .iter()
                .filter(|w| {
                    stats
                        .iter()
                        .all(|stt| *w.correct_a.get(stt.stat) > 0.0 && *w.required.get(stt.stat) <= stt.value)
                })
                .map(|wpn| {
                    (
                        calc_damage(wpn, &wpn_data, rstats, mixed_damage_scale.unwrap_or(1.0)),
                        wpn,
                    )
                })
                .collect::<Vec<_>>();
            wpns.sort_by(|a, b| b.0.score.partial_cmp(&a.0.score).unwrap());
            for (b, wpn) in wpns.iter().take(limit) {
                println!("{} - {b:?}", wpn.name);
            }
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
            let wpn_data = WeaponData::load_er(&regulations, &weapon_names)?;
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
            let best = soulsformats::optimize::best_stats(wpn, &wpn_data, two_handed, mins);
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
            let wpn_data = WeaponData::load_er(&regulations, &weapon_names)?;
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
            }
            let mut todo = Vec::new();
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
                        },
                        wpn,
                    ));
                }
            }

            if let Some(limit) = limit {
                // TODO: properly resize
                todo = todo.into_iter().take(limit).collect();
            }
            // reverse so that the slow ones (smithscript stuff) are computed first
            todo.reverse();

            eprintln!("TODO: {} elements", todo.len());

            let index: RwLock<Vec<WpnParams>> = RwLock::new(Vec::new());
            let done = AtomicUsize::new(0);
            let total = todo.len();

            todo.par_iter().for_each(|(params, wpn)| {
                let start = std::time::Instant::now();
                let thi = match params.handling {
                    THStatus::Any => "NI",
                    THStatus::OneHand => "1H",
                    THStatus::TwoHands => "2H",
                };
                let optim_result = soulsformats::optimize::best_stats(
                    wpn,
                    &wpn_data,
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
                let dn = done.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                println!("{dn}/{total} {:?} {:.2}s", params, elapsed.as_secs_f32())
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
