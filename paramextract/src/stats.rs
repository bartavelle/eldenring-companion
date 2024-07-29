use serde::Serialize;

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
    pub fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Stat<B> {
        Stat {
            str: f(self.str),
            dex: f(self.dex),
            int: f(self.int),
            fth: f(self.fth),
            arc: f(self.arc),
        }
    }
    pub fn fmap_r<B, F: Fn(&A) -> B>(&self, f: F) -> Stat<B> {
        Stat {
            str: f(&self.str),
            dex: f(&self.dex),
            int: f(&self.int),
            fth: f(&self.fth),
            arc: f(&self.arc),
        }
    }

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

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Damage<A> {
    pub physics: A,
    pub magic: A,
    pub fire: A,
    pub lightning: A,
    pub holy: A,
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
}

#[derive(Debug, Serialize)]
pub struct Effect<A> {
    pub poison: A,
    pub blood: A,
    pub sleep: A,
    pub madness: A,
}

impl<A> Effect<A> {
    pub fn fmap<B, F: Fn(A) -> B>(self, f: F) -> Effect<B> {
        Effect {
            poison: f(self.poison),
            blood: f(self.blood),
            sleep: f(self.sleep),
            madness: f(self.madness),
        }
    }

    pub fn map2<B, C, F: Fn(A, B) -> C>(self, other: Effect<B>, f: F) -> Effect<C> {
        Effect {
            poison: f(self.poison, other.poison),
            blood: f(self.blood, other.blood),
            sleep: f(self.sleep, other.sleep),
            madness: f(self.madness, other.madness),
        }
    }
}
