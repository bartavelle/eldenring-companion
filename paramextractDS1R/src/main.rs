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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let content = std::fs::read(args.params)?;
    let dec = decompress(content);
    let params: BND4 = soulsformats::formats::bnd3::BND3::parse(dec)?.into();

    match args.command {
        Command::WeaponDump => {
            let wpn_data = WeaponData::load_ds1r(&params)?;
            serde_json::to_writer_pretty(stdout(), &wpn_data)?;
        }
        Command::ArmorDump => todo!(),
    }

    Ok(())
}
