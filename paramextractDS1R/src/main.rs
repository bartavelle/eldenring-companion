use clap::{Parser, Subcommand, ValueEnum};
use ertypes::stats::{Stat, Statistic};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use soulsformats::formats::bnd4::BND4;
use soulsformats::formats::compression::decompress;
use soulsformats::formats::fmg::Fmg;
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
    #[arg(short, long, help = "Data dir")]
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
    ArmorDump,
    WeaponDump,
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
        #[arg(short, long, help = "Limit amount of compute (for testing)")]
        limit: Option<usize>,
    },
}

fn load_names(dir: &Path, name: &str) -> anyhow::Result<HashMap<u32, String>> {
    let mut dir = dir.to_owned();
    let mut namesmap: HashMap<String, u8> = HashMap::new();
    dir.push(name);
    let f = std::fs::File::open(dir)?;
    let b = BufReader::new(f);
    let mut out = HashMap::new();
    for l in b.lines() {
        let l = l?;
        match l.split_once('\t') {
            Some((rawid, nm)) => {
                let name = nm.trim().to_string();
                let name = if let Some(prev) = namesmap.insert(name.clone(), 0) {
                    let newname = format!("{} ({})", &name, prev + 1);
                    namesmap.insert(name, prev + 1);
                    newname
                } else {
                    name
                };
                let id = rawid.parse()?;
                out.insert(id, name);
            }
            None => panic!("malformed name line {l}"),
        }
    }
    Ok(out)
}

fn load_dcx(dir: &Path, name: &str) -> anyhow::Result<BND4> {
    let mut dir = dir.to_owned();
    dir.push(name);
    let content = std::fs::read(&dir)?;
    let dec = decompress(content);
    Ok(soulsformats::formats::bnd3::BND3::parse(dec)?.into())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let params = load_dcx(&args.data, "GameParam.parambnd.dcx")?;
    let itemsnames = load_dcx(&args.data, "item.msgbnd.dcx")?;

    eprintln!("version {}", params.version);

    match args.command {
        Command::WeaponDump => {
            let weapon_names = load_names(&args.data, "weapons.txt")?;
            let wpn_data = WeaponData::load_ds1r(&params, &weapon_names)?;
            serde_json::to_writer_pretty(stdout(), &wpn_data)?;
        }
        Command::ArmorDump => {
            let mut armor_names = Fmg::default();
            for (idx, fname) in itemsnames.file_names().iter().enumerate() {
                if fname.ends_with("Armor_name_.fmg") {
                    let anf = itemsnames.get_nth_file(idx).unwrap();
                    let cur_armor_names = Fmg::load(anf)?;
                    armor_names = armor_names.merge(cur_armor_names);
                }
            }

            let armor = soulsformats::armor::load_armor_ds1(&params, &armor_names.entries)?;
            let to_json = armor
                .into_values()
                .map(|armor| (armor.name.clone(), armor))
                .collect::<BTreeMap<_, _>>();
            serde_json::to_writer_pretty(stdout(), &to_json)?;
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
            let weapon_names = load_names(&args.data, "weapons.txt")?;
            let wpn_data = WeaponData::load_ds1r(&params, &weapon_names)?;
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
            let weapon_names = load_names(&args.data, "weapons.txt")?;
            let wpn_data = WeaponData::load_ds1r(&params, &weapon_names)?;
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
