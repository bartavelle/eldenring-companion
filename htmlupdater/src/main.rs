use eldenring_companion::{Absorptions, Armor, ArmorCategory, Body, Resistances, Scored, Weights};
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "htmlupdater",
    about = "Updates json files with html from fextralife (elden ring only)"
)]
struct Opt {
    /// Path to the data dir containing the .json files
    #[structopt(long)]
    oldjson: PathBuf,
    #[structopt(long)]
    htmlfiles: PathBuf,
}

fn load_old_data(dir: &Path) -> HashMap<String, Armor> {
    let mut p = dir.to_path_buf();
    p.push("armor.json");
    let f_armor = std::fs::File::open(p).unwrap();
    serde_json::from_reader(f_armor).unwrap()
}

fn flt(n: ego_tree::NodeRef<scraper::Node>) -> f64 {
    let Some(chld) = n.first_child() else {
        panic!("no child for {:?}", n.value());
    };
    let Some(txt) = chld.value().as_text() else {
        panic!("not a text element for {:?}", chld.value());
    };
    match txt.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("invalid text {txt:?}"),
    }
}

fn main() {
    let args = Opt::from_args();
    let mut old_game_data = load_old_data(&args.oldjson);
    for (category, fname) in [
        (ArmorCategory::Arms, "Gauntlets"),
        (ArmorCategory::Body, "Chest Armor"),
        (ArmorCategory::Head, "Helms"),
        (ArmorCategory::Legs, "Leg Armor"),
    ] {
        eprintln!("parsing {category:?}");
        let mut pth = args.htmlfiles.clone();
        pth.push(fname);
        let mut f_part = std::fs::File::open(pth).unwrap();
        let mut content = Vec::new();
        f_part.read_to_end(&mut content).unwrap();
        let dom = scraper::html::Html::parse_fragment(&String::from_utf8_lossy(&content));
        let tbl = dom.root_element().children().find(|c| c.value().is_element()).unwrap();
        let body = tbl
            .children()
            .find(|c| c.value().as_element().map(|x| x.name() == "tbody").unwrap_or(false))
            .unwrap();
        for c in body
            .children()
            .filter(|x| x.value().as_element().map(|x| x.name() == "tr").unwrap_or(false))
        {
            let mut parts = c.children().filter(|e| e.value().is_element());
            let raw_name = parts.next().unwrap();
            let Some(name_chld) = raw_name.children().find(|x| x.value().is_element()) else {
                panic!("no children for {:?}", raw_name.value())
            };
            let name = match name_chld.value() {
                scraper::Node::Document => todo!(),
                scraper::Node::Fragment => todo!(),
                scraper::Node::Doctype(_) => todo!(),
                scraper::Node::Comment(_) => todo!(),
                scraper::Node::Text(_) => todo!(),
                scraper::Node::Element(name_element) => match name_element.name() {
                    "a" => name_element.attr("title").unwrap(),
                    "p" => name_chld
                        .first_child()
                        .unwrap()
                        .value()
                        .as_element()
                        .unwrap()
                        .attr("title")
                        .unwrap(),
                    d => panic!("unsupported element {d}"),
                },
                scraper::Node::ProcessingInstruction(_) => todo!(),
            };
            let name = name.strip_prefix("Elden Ring").unwrap_or(name).trim();
            eprintln!("  {name}");
            let raw_phy = flt(parts.next().unwrap());
            let raw_str = flt(parts.next().unwrap());
            let raw_sla = flt(parts.next().unwrap());
            let raw_pie = flt(parts.next().unwrap());
            let raw_mag = flt(parts.next().unwrap());
            let raw_fir = flt(parts.next().unwrap());
            let raw_lit = flt(parts.next().unwrap());
            let raw_hol = flt(parts.next().unwrap());
            let raw_imm = flt(parts.next().unwrap());
            let raw_rob = flt(parts.next().unwrap());
            let raw_foc = flt(parts.next().unwrap());
            let raw_vit = flt(parts.next().unwrap());
            let raw_poi = flt(parts.next().unwrap());
            let raw_wgt = flt(parts.next().unwrap());
            let armor = Armor {
                category,
                name: name.to_string(),
                weight: raw_wgt,
                absorptions: Absorptions {
                    fire: raw_fir,
                    holy: raw_hol,
                    lightning: raw_lit,
                    magic: raw_mag,
                    physical: raw_phy,
                    pierce: raw_pie,
                    slash: raw_sla,
                    strike: raw_str,
                },
                resistances: Resistances {
                    focus: raw_foc,
                    immunity: raw_imm,
                    poise: raw_poi,
                    robustness: raw_rob,
                    vitality: raw_vit,
                },
            };
            old_game_data.entry(name.to_string()).or_insert(armor);
        }
    }
    println!("{}", serde_json::to_string(&old_game_data).unwrap());
}
