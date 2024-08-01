use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Stat<A> {
    pub str: A,
    pub dex: A,
    pub int: A,
    pub fth: A,
    pub arc: A,
}

impl<A: Copy> Stat<A> {
    pub fn all(a: A) -> Self {
        Stat {
            str: a,
            dex: a,
            int: a,
            fth: a,
            arc: a,
        }
    }
}

impl<A> Stat<A> {
    pub fn map2<B, C, F: Fn(A, B) -> C>(self, other: Stat<B>, f: F) -> Stat<C> {
        Stat {
            str: f(self.str, other.str),
            dex: f(self.dex, other.dex),
            int: f(self.int, other.int),
            fth: f(self.fth, other.fth),
            arc: f(self.arc, other.arc),
        }
    }

    pub fn map2_r<B, C, F: Fn(&A, &B) -> C>(&self, other: &Stat<B>, f: F) -> Stat<C> {
        Stat {
            str: f(&self.str, &other.str),
            dex: f(&self.dex, &other.dex),
            int: f(&self.int, &other.int),
            fth: f(&self.fth, &other.fth),
            arc: f(&self.arc, &other.arc),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct Damage<A> {
    pub physics: A,
    pub magic: A,
    pub fire: A,
    pub lightning: A,
    pub holy: A,
}

impl<A: PartialOrd> PartialOrd for Damage<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.physics >= other.physics
            && self.magic >= other.magic
            && self.fire >= other.fire
            && self.lightning >= other.lightning
            && self.holy >= other.holy
        {
            Some(std::cmp::Ordering::Greater)
        } else if self.physics <= other.physics
            && self.magic <= other.magic
            && self.fire <= other.fire
            && self.lightning <= other.lightning
            && self.holy <= other.holy
        {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl<A> Damage<A> {
    pub fn fmap_r<B, F: Fn(&A) -> B>(&self, f: F) -> Damage<B> {
        Damage {
            physics: f(&self.physics),
            magic: f(&self.magic),
            fire: f(&self.fire),
            lightning: f(&self.lightning),
            holy: f(&self.holy),
        }
    }

    pub fn map2<B, C, F: Fn(A, B) -> C>(self, other: Damage<B>, f: F) -> Damage<C> {
        Damage {
            physics: f(self.physics, other.physics),
            magic: f(self.magic, other.magic),
            fire: f(self.fire, other.fire),
            lightning: f(self.lightning, other.lightning),
            holy: f(self.holy, other.holy),
        }
    }

    pub fn map2_r<B, C, F: Fn(&A, &B) -> C>(&self, other: &Damage<B>, f: F) -> Damage<C> {
        Damage {
            physics: f(&self.physics, &other.physics),
            magic: f(&self.magic, &other.magic),
            fire: f(&self.fire, &other.fire),
            lightning: f(&self.lightning, &other.lightning),
            holy: f(&self.holy, &other.holy),
        }
    }

    pub fn to_slice(&self) -> [&A; 5] {
        [&self.physics, &self.magic, &self.fire, &self.lightning, &self.holy]
    }
}

#[derive(Debug, Serialize)]
pub struct Effect<A> {
    pub blood: A,
    pub frost: A,
    pub sleep: A,
    pub scarlet_rot: A,
    pub madness: A,
    pub poison: A,
}

impl<A> Effect<A> {
    pub fn fmap_r<B, F: Fn(&A) -> B>(&self, f: F) -> Effect<B> {
        Effect {
            poison: f(&self.poison),
            blood: f(&self.blood),
            sleep: f(&self.sleep),
            madness: f(&self.madness),
            frost: f(&self.frost),
            scarlet_rot: f(&self.scarlet_rot),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Passive {
    Blood,
    Frost,
    Sleep,
    ScarletRot,
    Madness,
    Poison,
}

#[derive(Debug, Clone)]
pub struct Reinforcement {
    pub damage: Damage<f32>,
    pub stats: Stat<f32>,
    pub sp0: u8,
    pub sp1: u8,
    pub sp2: u8,
}
