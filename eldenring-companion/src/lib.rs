mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Serialize, Deserialize)]
enum ArmorCategory {
    Head,
    Arms,
    Body,
    Legs,
}

#[derive(Serialize, Deserialize)]
struct Absorptions<A> {
    fire: A,
    holy: A,
    lightning: A,
    magic: A,
    physical: A,
    pierce: A,
    slash: A,
    strike: A,
}

#[derive(Serialize, Deserialize)]
struct Armor {
    category: ArmorCategory,
    name: String,
    weight: f64,
    absorptions: Absorptions<i64>,
    resistances: Resistances<i64>,
}

#[derive(Serialize, Deserialize)]
struct Resistances<A> {
    focus: A,
    immunity: A,
    poise: A,
    robustness: A,
    vitality: A,
}

#[derive(Serialize, Deserialize)]
struct Body<A> {
    head: A,
    arms: A,
    body: A,
    legs: A,
}

impl<A> Body<Vec<A>> {
    fn fmap<B, F>(self, f: F) -> Body<Vec<B>>
    where
        F: Fn(A) -> B,
    {
        Body {
            head: self.head.into_iter().map(&f).collect(),
            arms: self.arms.into_iter().map(&f).collect(),
            body: self.body.into_iter().map(&f).collect(),
            legs: self.legs.into_iter().map(&f).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Weights {
    absorptions: Absorptions<f64>,
    resistances: Resistances<f64>,
}

#[derive(Serialize, Deserialize)]
struct Scored {
    name: String,
    weight: f64,
    score: f64,
}

fn scores(i: Body<Vec<Armor>>, w: &Weights) -> Body<Vec<Scored>> {
    let score = |a: &Armor| {
        a.resistances.focus as f64 * w.resistances.focus
            + a.resistances.immunity as f64 * w.resistances.immunity
            + a.resistances.poise as f64 * w.resistances.poise
            + a.resistances.robustness as f64 * w.resistances.robustness
            + a.resistances.vitality as f64 * w.resistances.vitality
            + a.absorptions.fire as f64 * w.absorptions.fire
            + a.absorptions.holy as f64 * w.absorptions.holy
            + a.absorptions.lightning as f64 * w.absorptions.lightning
            + a.absorptions.magic as f64 * w.absorptions.magic
            + a.absorptions.physical as f64 * w.absorptions.physical
            + a.absorptions.pierce as f64 * w.absorptions.pierce
            + a.absorptions.slash as f64 * w.absorptions.slash
            + a.absorptions.strike as f64 * w.absorptions.strike
    };
    i.fmap(|armor| {
        let score = score(&armor);
        Scored {
            name: armor.name,
            score,
            weight: armor.weight,
        }
    })
}

struct NextComparer {
    curvec: Vec<Scored>,
    curel: Option<Scored>,
}

impl NextComparer {
    fn new() -> Self {
        NextComparer {
            curvec: Vec::new(),
            curel: None,
        }
    }

    fn finalize(self) -> Vec<Scored> {
        let mut curvec = self.curvec;
        if let Some(e) = self.curel {
            curvec.push(e)
        }
        curvec
    }

    fn next(self, s: Scored) -> Self {
        match self.curel {
            None => Self {
                curvec: self.curvec,
                curel: Some(s),
            },
            Some(prev) => {
                if prev.weight <= s.weight {
                    Self {
                        curvec: self.curvec,
                        curel: Some(prev),
                    }
                } else {
                    let mut curvec = self.curvec;
                    curvec.push(prev);
                    Self {
                        curvec,
                        curel: Some(s),
                    }
                }
            }
        }
    }
}

fn prepare_list(i: Vec<Scored>) -> Vec<Scored> {
    let mut i = i;
    i.push(Scored {
        name: "Nothing".into(),
        weight: 0.0,
        score: 0.0,
    });
    i.sort_unstable_by(|a, b| {
        b.score.partial_cmp(&a.score).unwrap_or_else(|| {
            a.weight
                .partial_cmp(&b.weight)
                .unwrap_or_else(|| a.name.cmp(&b.name))
        })
    });

    i.into_iter()
        .fold(NextComparer::new(), |acc, n| acc.next(n))
        .finalize()
}

fn search(i: Body<Vec<Scored>>, weight_budget: f64) -> Body<String> {
    // first, sort by scores
    let head = prepare_list(i.head);
    let arms = prepare_list(i.arms);
    let body = prepare_list(i.body);
    let legs = prepare_list(i.legs);

    let mut best = Body {
        head: "Empty".into(),
        arms: "Empty".into(),
        body: "Empty".into(),
        legs: "Empty".into(),
    };
    let mut best_score = 0.0;
    for b in &body {
        let bbudget = weight_budget - b.weight;
        if bbudget < 0.0 {
            continue;
        }
        for l in &legs {
            let lbudget = bbudget - l.weight;
            if lbudget < 0.0 {
                continue;
            }
            for a in &arms {
                let abudget = lbudget - a.weight;
                if abudget < 0.0 {
                    continue;
                }
                for h in &head {
                    let hbudget = abudget - a.weight;
                    if hbudget < 0.0 {
                        continue;
                    }
                    // we got the best head right now
                    let score = b.score + l.score + a.score + h.score;
                    if score < best_score {
                        break;
                    }
                    best_score = score;
                    best = Body {
                        head: h.name.clone(),
                        arms: a.name.clone(),
                        body: b.name.clone(),
                        legs: l.name.clone(),
                    }
                }
            }
        }
    }

    best
}

#[wasm_bindgen]
pub fn optimize_armor(armors: JsValue, weights: JsValue, weight_budget: f64) -> JsValue {
    let armors: Body<Vec<Armor>> = serde_wasm_bindgen::from_value(armors).unwrap();
    let weights: Weights = serde_wasm_bindgen::from_value(weights).unwrap();

    let warmor = scores(armors, &weights);

    alert("Hello, eldenring-companion!");

    let res = search(warmor, weight_budget);

    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prepare_list_a() {
        let e1 = Scored {
            name: "e1".to_string(),
            weight: 5.0,
            score: 5.0,
        };
        let e2 = Scored {
            name: "e2".to_string(),
            weight: 4.0,
            score: 4.0,
        };
        let e3 = Scored {
            name: "e3".to_string(),
            weight: 3.0,
            score: 3.0,
        };
        let e4 = Scored {
            name: "e4".to_string(),
            weight: 3.0,
            score: 2.0,
        };
        let e5 = Scored {
            name: "e5".to_string(),
            weight: 1.0,
            score: 1.0,
        };
        let o = prepare_list(vec![e3, e5, e1, e2, e4])
            .into_iter()
            .map(|s| s.score)
            .collect::<Vec<_>>();
        assert_eq!(o, [5.0, 4.0, 3.0, 1.0, 0.0])
    }
}
