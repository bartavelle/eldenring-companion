use eldenring_companion::{scores, search, Absorptions, Body, Resistances, Scored, Weights};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::gamedata::load_data;

pub mod gamedata;

#[derive(Debug, StructOpt)]
enum Mode {
    Svg,
    Ratio,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "optimizer",
    about = "Spits various statistics about elden ring"
)]
struct Opt {
    /// Path to the data dir containing the .json files
    #[structopt(long)]
    datadir: PathBuf,
    #[structopt(flatten)]
    weights: VWeights,
    /// maximum weight budget
    #[structopt(long, default_value = "65")]
    max: f64,
    #[structopt(subcommand)]
    mode: Mode,
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

    match args.mode {
        Mode::Svg => svg_chart(&warmor, args.max),
        Mode::Ratio => ratio(&warmor),
    }
}

fn ratio(warmor: &Body<Vec<Scored>>) {
    let mut all_armor = warmor
        .body
        .iter()
        .chain(warmor.arms.iter())
        .chain(warmor.legs.iter())
        .chain(warmor.head.iter())
        .cloned()
        .collect::<Vec<_>>();
    all_armor.sort_by(|a, b| {
        let sa = a.score / a.weight;
        let sb = b.score / b.weight;
        sa.total_cmp(&sb)
    });
    all_armor.reverse();
    for a in all_armor {
        println!("{} - {:.2} - {:.1}", a.name, a.score, a.weight);
    }
}

fn svg_chart(warmor: &Body<Vec<Scored>>, max_weight: f64) {
    println!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="1920" height="1080" viewBox="0 0 1920 1080" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<style>
    .small {{
      font: 20px sans-serif;
      color: black;
    }}
    line {{
        stroke: black;
    }}
    .gline {{
        stroke: grey;
    }}
    rect {{
        fill: #5fa;
        stroke: #c5a;
    }}
</style>
<line x1="50" y1="50" x2="1870" y2="50"/>
<line x1="50" y1="1030" x2="1870" y2="1030"/>
<line x1="50" y1="50" x2="50" y2="1030"/>
<line x1="1870" y1="50" x2="1870" y2="1030"/>
"#
    );
    let (_, max_score) = search(warmor, max_weight);

    let w = |n: f64| 50.0 + n * 1820.0 / max_weight;
    let h = |n: f64| 50.0 + (max_score - n) * 980.0 / max_score;

    {
        let mut sc = 0.0;
        while sc < max_score {
            let lh = h(sc);
            println!(
                r#"<text x="5" y="{}" class="small">{}</text>"#,
                lh + 7.0,
                sc
            );
            println!(
                r#"<line x1="40" x2="50" y1="{}" y2="{}"/>"#,
                lh, lh
            );
            println!(
                r#"<line x1="50" x2="1930" y1="{}" y2="{}" class="gline" stroke-dasharray="20 40"/>"#,
                lh, lh
            );
            sc += 5.0;
        }
    }
    {
        let mut incr = 0.0;
        while incr <= max_weight {
            let lw = w(incr);
            println!(
                r#"<line y1="1030" y2="1040" x1="{}" x2="{}"/>"#,
                lw, lw
            );
            println!(
                r#"<text y="1060" x="{}" class="small">{}</text>"#,
                lw - if incr < 10.0 { 5.0 } else { 11.0 },
                incr
            );
            incr += 10.0;
        }
    }

    let mut weight_budget = 0.0;
    let mut best_score = 0.0;
    let mut curbest: Option<Body<String>> = None;
    let mut prev_weight = 0.0;

    while weight_budget <= max_weight {
        let (best, score) = search(warmor, weight_budget);
        if score > best_score {
            if let Some(pbest) = &curbest {
                println!(
                    r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}">
    <title>{}, {}, {}, {}, weight={:.2}</title>
 </rect>"##,
                    w(prev_weight),
                    h(score),
                    w(weight_budget) - w(prev_weight),
                    1030.0 - h(score),
                    &pbest.head,
                    &pbest.body,
                    &pbest.arms,
                    &pbest.legs,
                    prev_weight,
                );
            }
            best_score = score;
            curbest = Some(best);
            prev_weight = weight_budget;
        }
        weight_budget += 0.1;
    }
    if let Some(pbest) = &curbest {
        println!(
            r##"<rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}">
       <title>{}, {}, {}, {}, weight={:.1}</title>
    </rect>"##,
            w(prev_weight),
            h(best_score),
            w(max_weight) - w(prev_weight),
            1030.0 - h(best_score),
            &pbest.head,
            &pbest.body,
            &pbest.arms,
            &pbest.legs,
            prev_weight,
        );
    }
    println!("</svg>");
}
