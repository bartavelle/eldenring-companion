mod utils;

use std::cmp::Ordering;

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorCategory {
    Head,
    Arms,
    Body,
    Legs,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Absorptions<A> {
    pub fire: A,
    pub holy: A,
    pub lightning: A,
    pub magic: A,
    pub physical: A,
    pub pierce: A,
    pub slash: A,
    pub strike: A,
}

impl<A: Copy> Absorptions<A> {
    fn all(a: A) -> Self {
        Absorptions {
            fire: a,
            holy: a,
            lightning: a,
            magic: a,
            physical: a,
            pierce: a,
            slash: a,
            strike: a,
        }
    }

    fn map2<B, C, F>(self, other: Absorptions<B>, f: F) -> Absorptions<C>
    where
        F: Fn(A, B) -> C,
    {
        Absorptions {
            fire: f(self.fire, other.fire),
            holy: f(self.holy, other.holy),
            lightning: f(self.lightning, other.lightning),
            magic: f(self.magic, other.magic),
            physical: f(self.physical, other.physical),
            pierce: f(self.pierce, other.pierce),
            slash: f(self.slash, other.slash),
            strike: f(self.strike, other.strike),
        }
    }
}

impl Absorptions<f64> {
    fn sum(&self) -> f64 {
        self.fire + self.holy + self.lightning + self.magic + self.physical + self.pierce + self.slash + self.strike
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Armor {
    pub category: ArmorCategory,
    pub name: String,
    pub weight: f64,
    pub absorptions: Absorptions<f64>,
    pub resistances: Resistances<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Resistances<A> {
    pub focus: A,
    pub immunity: A,
    pub poise: A,
    pub robustness: A,
    pub vitality: A,
}
impl Resistances<f64> {
    fn sum(&self) -> f64 {
        self.focus + self.immunity + self.poise + self.robustness + self.vitality
    }
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Body<A> {
    pub head: A,
    pub arms: A,
    pub body: A,
    pub legs: A,
}

impl<A> Body<Vec<A>> {
    pub fn fmap<B, F>(self, f: F) -> Body<Vec<B>>
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
pub struct Weights {
    pub absorptions: Absorptions<f64>,
    pub resistances: Resistances<f64>,
}
impl Weights {
    fn sum(&self) -> f64 {
        self.absorptions.sum() + self.resistances.sum()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Scored {
    pub name: String,
    pub weight: f64,
    pub score: f64, // individual part score
    pub inv_absorptions: Absorptions<f64>,
    pub resistances_score: f64,
}

pub fn scores(i: Body<Vec<Armor>>, w: &Weights) -> Body<Vec<Scored>> {
    let total_weights = w.sum();
    if total_weights == 0.0 {
        return Body::default();
    }
    i.fmap(|armor| {
        let resistances_score = armor.resistances.focus * w.resistances.focus
            + armor.resistances.immunity * w.resistances.immunity
            + armor.resistances.poise * w.resistances.poise
            + armor.resistances.robustness * w.resistances.robustness
            + armor.resistances.vitality * w.resistances.vitality;

        let score = (resistances_score
            + armor.absorptions.fire * w.absorptions.fire
            + armor.absorptions.holy * w.absorptions.holy
            + armor.absorptions.lightning * w.absorptions.lightning
            + armor.absorptions.magic * w.absorptions.magic
            + armor.absorptions.physical * w.absorptions.physical
            + armor.absorptions.pierce * w.absorptions.pierce
            + armor.absorptions.slash * w.absorptions.slash
            + armor.absorptions.strike * w.absorptions.strike)
            / total_weights;
        Scored {
            name: armor.name,
            score,
            weight: armor.weight,
            inv_absorptions: Absorptions {
                fire: 1.0 - (armor.absorptions.fire / 100.0),
                holy: 1.0 - (armor.absorptions.holy / 100.0),
                lightning: 1.0 - (armor.absorptions.lightning / 100.0),
                magic: 1.0 - (armor.absorptions.magic / 100.0),
                physical: 1.0 - (armor.absorptions.physical / 100.0),
                pierce: 1.0 - (armor.absorptions.pierce / 100.0),
                slash: 1.0 - (armor.absorptions.slash / 100.0),
                strike: 1.0 - (armor.absorptions.strike / 100.0),
            },
            resistances_score,
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
                    Self { curvec, curel: Some(s) }
                }
            }
        }
    }
}

fn prepare_list(i: &[Scored]) -> Vec<Scored> {
    let mut i = i.to_vec();
    i.push(Scored {
        name: "Nothing".into(),
        weight: 0.0,
        score: 0.0,
        inv_absorptions: Absorptions::all(1.0),
        resistances_score: 0.0,
    });
    i.sort_unstable_by(|a, b| match b.score.partial_cmp(&a.score) {
        Some(Ordering::Greater) => Ordering::Greater,
        Some(Ordering::Less) => Ordering::Less,
        _ => match a.weight.partial_cmp(&b.weight) {
            Some(Ordering::Greater) => Ordering::Greater,
            Some(Ordering::Less) => Ordering::Less,
            _ => a.name.cmp(&b.name),
        },
    });

    i.into_iter().fold(NextComparer::new(), |acc, n| acc.next(n)).finalize()
}

fn compound_resistances(rs: &[Absorptions<f64>]) -> Absorptions<f64> {
    let multiplied = rs
        .iter()
        .fold(Absorptions::all(1.0), |cur, n| cur.map2(*n, |a, b| a * b));
    Absorptions {
        fire: (1.0 - multiplied.fire) * 100.0,
        holy: (1.0 - multiplied.holy) * 100.0,
        lightning: (1.0 - multiplied.lightning) * 100.0,
        magic: (1.0 - multiplied.magic) * 100.0,
        physical: (1.0 - multiplied.physical) * 100.0,
        pierce: (1.0 - multiplied.pierce) * 100.0,
        slash: (1.0 - multiplied.slash) * 100.0,
        strike: (1.0 - multiplied.strike) * 100.0,
    }
}

fn score_multiple_resistances(rs: &[Absorptions<f64>], w: &Weights) -> f64 {
    let acc = compound_resistances(rs);
    let mul = acc.map2(w.absorptions, |a, b| a * b);
    mul.fire + mul.holy + mul.lightning + mul.magic + mul.physical + mul.pierce + mul.slash + mul.strike
}

pub fn search(i: &Body<Vec<Scored>>, w: &Weights, weight_budget: f64) -> (Body<String>, f64) {
    // first, sort by scores
    let head = prepare_list(&i.head);
    let arms = prepare_list(&i.arms);
    let body = prepare_list(&i.body);
    let legs = prepare_list(&i.legs);

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
        let bscore = b.resistances_score + score_multiple_resistances(&[b.inv_absorptions], w);
        if bscore > best_score {
            best = Body {
                head: "Empty".to_string(),
                arms: "Empty".to_string(),
                body: b.name.clone(),
                legs: "Empty".to_string(),
            }
        }
        for l in &legs {
            let lbudget = bbudget - l.weight;
            if lbudget < 0.0 {
                continue;
            }
            let lscore = b.resistances_score
                + l.resistances_score
                + score_multiple_resistances(&[b.inv_absorptions, l.inv_absorptions], w);
            if lscore > best_score {
                best = Body {
                    head: "Empty".to_string(),
                    arms: "Empty".to_string(),
                    body: b.name.clone(),
                    legs: l.name.clone(),
                }
            }
            for a in &arms {
                let abudget = lbudget - a.weight;
                if abudget < 0.0 {
                    continue;
                }
                let ascore = b.resistances_score
                    + l.resistances_score
                    + a.resistances_score
                    + score_multiple_resistances(&[b.inv_absorptions, l.inv_absorptions, a.inv_absorptions], w);
                if ascore > best_score {
                    best = Body {
                        head: "Empty".to_string(),
                        arms: a.name.clone(),
                        body: b.name.clone(),
                        legs: l.name.clone(),
                    }
                }
                for h in &head {
                    let hbudget = abudget - h.weight;
                    if hbudget < 0.0 {
                        continue;
                    }
                    // we got the best head right now
                    let hscore = b.resistances_score
                        + l.resistances_score
                        + a.resistances_score
                        + h.resistances_score
                        + score_multiple_resistances(
                            &[
                                b.inv_absorptions,
                                l.inv_absorptions,
                                a.inv_absorptions,
                                h.inv_absorptions,
                            ],
                            w,
                        );
                    if hscore < best_score {
                        break;
                    }
                    best_score = hscore;
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

    (best, best_score / w.sum())
}

#[wasm_bindgen]
pub fn optimize_armor(armors: JsValue, weights: JsValue, weight_budget: f64) -> JsValue {
    let armors: Body<Vec<Armor>> = serde_wasm_bindgen::from_value(armors).unwrap();
    let weights: Weights = serde_wasm_bindgen::from_value(weights).unwrap();

    let warmor = scores(armors, &weights);
    let res = search(&warmor, &weights, weight_budget);

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
            inv_absorptions: Absorptions {
                fire: 1.0,
                holy: 1.0,
                lightning: 1.0,
                magic: 1.0,
                physical: 1.0,
                pierce: 1.0,
                slash: 1.0,
                strike: 1.0,
            },
            resistances_score: 0.0,
        };
        let e2 = Scored {
            name: "e2".to_string(),
            weight: 4.0,
            score: 4.0,
            inv_absorptions: Absorptions {
                fire: 1.0,
                holy: 1.0,
                lightning: 1.0,
                magic: 1.0,
                physical: 1.0,
                pierce: 1.0,
                slash: 1.0,
                strike: 1.0,
            },
            resistances_score: 0.0,
        };
        let e3 = Scored {
            name: "e3".to_string(),
            weight: 3.0,
            score: 3.0,
            inv_absorptions: Absorptions {
                fire: 1.0,
                holy: 1.0,
                lightning: 1.0,
                magic: 1.0,
                physical: 1.0,
                pierce: 1.0,
                slash: 1.0,
                strike: 1.0,
            },
            resistances_score: 0.0,
        };
        let e4 = Scored {
            name: "e4".to_string(),
            weight: 3.0,
            score: 2.0,
            inv_absorptions: Absorptions {
                fire: 1.0,
                holy: 1.0,
                lightning: 1.0,
                magic: 1.0,
                physical: 1.0,
                pierce: 1.0,
                slash: 1.0,
                strike: 1.0,
            },
            resistances_score: 0.0,
        };
        let e5 = Scored {
            name: "e5".to_string(),
            weight: 1.0,
            score: 1.0,
            inv_absorptions: Absorptions {
                fire: 1.0,
                holy: 1.0,
                lightning: 1.0,
                magic: 1.0,
                physical: 1.0,
                pierce: 1.0,
                slash: 1.0,
                strike: 1.0,
            },
            resistances_score: 0.0,
        };
        let o = prepare_list(&[e3, e5, e1, e2, e4])
            .into_iter()
            .map(|s| s.score)
            .collect::<Vec<_>>();
        assert_eq!(o, [5.0, 4.0, 3.0, 1.0, 0.0])
    }
}
