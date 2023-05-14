// capital letters to fit the source json
type ArmorSlots<A> = {
  Arms: A,
  Body: A,
  Head: A,
  Legs: A,
}

type F1<A, B> = (a: A) => B;
type F2<A, B, C> = (a: A, b: B) => C;

function Azipwith<A, B, C>(f: F2<A, B, C>, as: ArmorSlots<A>, bs: ArmorSlots<B>): ArmorSlots<C> {
  return {
    Arms: f(as.Arms, bs.Arms),
    Body: f(as.Body, bs.Body),
    Head: f(as.Head, bs.Head),
    Legs: f(as.Legs, bs.Legs),
  }
}

function Amap<A, B>(f: F1<A, B>, as: ArmorSlots<A>): ArmorSlots<B> {
  return {
    Arms: f(as.Arms),
    Body: f(as.Body),
    Head: f(as.Head),
    Legs: f(as.Legs),
  }
}

function Ainit<A>(a: A): ArmorSlots<A> {
  return {
    Arms: a,
    Body: a,
    Head: a,
    Legs: a
  }
}

export { type ArmorSlots, type F1, type F2, Azipwith, Amap, Ainit }
