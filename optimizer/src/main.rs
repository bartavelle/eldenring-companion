use eldenring_companion::{scores, search, Absorptions, Resistances, Weights};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::gamedata::load_data;

pub mod gamedata;

#[derive(Debug, StructOpt)]
#[structopt(name = "optimizer", about = "Spits various statistics about elden ring")]
struct Opt {
    /// Path to the data dir containing the .json files
    #[structopt(long)]
    datadir: PathBuf,
    #[structopt(flatten)]
    weights: VWeights,
}

#[derive(Debug, StructOpt)]
#[structopt()]
struct VWeights {
    #[structopt(long, default_value = "0.0")]
    fire: f64,
    #[structopt(long, default_value = "0.0")]
    holy: f64,
    #[structopt(long, default_value = "0.0")]
    lightning: f64,
    #[structopt(long, default_value = "0.0")]
    magic: f64,
    #[structopt(long, default_value = "1.0")]
    physical: f64,
    #[structopt(long, default_value = "0.0")]
    pierce: f64,
    #[structopt(long, default_value = "0.0")]
    slash: f64,
    #[structopt(long, default_value = "0.0")]
    strike: f64,
    #[structopt(long, default_value = "0.0")]
    focus: f64,
    #[structopt(long, default_value = "0.0")]
    immunity: f64,
    #[structopt(long, default_value = "0.0")]
    poise: f64,
    #[structopt(long, default_value = "0.0")]
    robustness: f64,
    #[structopt(long, default_value = "0.0")]
    vitality: f64,
}

impl From<VWeights> for Weights {
    fn from(w: VWeights) -> Self {
        Weights {
            absorptions: Absorptions {
                fire: w.fire,
                holy: w.holy,
                lightning: w.lightning,
                magic: w.magic,
                physical: w.physical,
                pierce: w.pierce,
                slash: w.slash,
                strike: w.strike,
            },
            resistances: Resistances {
                focus: w.focus,
                immunity: w.immunity,
                poise: w.poise,
                robustness: w.robustness,
                vitality: w.vitality,
            },
        }
    }
}

fn main() {
    let args = Opt::from_args();
    let game_data = load_data(&args.datadir);
    let weights: Weights = args.weights.into();

    let warmor = scores(game_data.armors, &weights);
    let mut weight_budget = 0.0;
    let mut best_score = 0.0;
    while weight_budget < 100.0 {
        let (best, score) = search(&warmor, weight_budget);
        if score != best_score {
            println!("{:2.1} {:2.1} - {:?}", weight_budget, score, best);
            best_score = score;
        }
        weight_budget += 0.1;
    }
}
