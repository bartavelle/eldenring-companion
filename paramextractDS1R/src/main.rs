use clap::{Parser, Subcommand, ValueEnum};
use ertypes::stats::{Stat, Statistic};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use soulsformats::formats::bnd4::BND4;
use soulsformats::formats::compression::decompress;
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
    #[arg(short, long, help = "Param file")]
    params: PathBuf,
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
}

fn load_names(dir: &Path, name: &str) -> anyhow::Result<HashMap<u32, String>> {
    let mut dir = dir.to_owned();
    dir.push(name);
    let f = std::fs::File::open(dir)?;
    let b = BufReader::new(f);
    let mut out = HashMap::new();
    for l in b.lines() {
        let l = l?;
        match l.split_once('\t') {
            Some((rawid, nm)) => {
                let id = rawid.parse()?;
                out.insert(id, nm.trim().to_string());
            }
            None => panic!("malformed name line {l}"),
        }
    }
    Ok(out)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let content = std::fs::read(args.params)?;
    let dec = decompress(content);
    let params: BND4 = soulsformats::formats::bnd3::BND3::parse(dec)?.into();

    match args.command {
        Command::WeaponDump => {
            let weapon_names = load_names(&args.data, "weapons.txt")?;
            let wpn_data = WeaponData::load_ds1r(&params, &weapon_names)?;
            serde_json::to_writer_pretty(stdout(), &wpn_data)?;
        }
        Command::ArmorDump => {
            let armor = soulsformats::armor::load_armor_ds1(&params)?;
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
    }

    Ok(())
}
