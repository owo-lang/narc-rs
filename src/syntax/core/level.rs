use super::{Closure, Neutral, Val};
use voile_util::level::{
    calc_slice_plus_one_level, calc_tree_map_level, calc_tree_map_plus_one_level, lift_tree_map,
    Level, LevelCalcState, LiftEx,
};

pub const TYPE_OMEGA: Val = Val::Type(Level::Omega);

impl LiftEx for Val {
    fn lift(self, levels: u32) -> Val {
        match self {
            Val::Type(l) => Val::Type(l + levels),
            Val::Lam(closure) => Val::Lam(closure.lift(levels)),
            Val::Pi(plicit, param_type, closure) => {
                Val::pi(plicit, param_type.lift(levels), closure.lift(levels))
            }
            Val::Cons(name, e) => Val::cons(name, e.lift(levels)),
            Val::Neut(neut) => Val::Neut(neut.lift(levels)),
        }
    }

    fn calc_level(&self) -> LevelCalcState {
        match self {
            Val::Type(level) => Some(*level + 1),
            Val::Pi(_, param_ty, closure) => {
                Some(param_ty.calc_level()?.max(closure.calc_level()?))
            }
            Val::Lam(closure) => closure.calc_level(),
            Val::Neut(neut) => neut.calc_level(),
            Val::Cons(_, e) => e.calc_level(),
        }
    }
}

impl LiftEx for Neutral {
    fn lift(self, levels: u32) -> Self {
        use super::Neutral::*;
        match self {
            Lift(n, expr) => Lift(n + levels, expr),
            e => Lift(levels, Box::new(e)),
        }
    }

    fn calc_level(&self) -> LevelCalcState {
        use super::Neutral::*;
        match self {
            Lift(n, expr) => match expr.calc_level() {
                Some(m) => Some(m + *n),
                // Trying to lift yourself makes you omega.
                None => Some(Level::Omega),
            },
            // Level is zero by default
            Var(..) | Axi(..) | Meta(..) => Some(Default::default()),
            Ref(..) => None,
            App(f, args) => calc_slice_plus_one_level(&**f, args),
            SplitOn(split, on) => calc_tree_map_plus_one_level(&**on, split),
        }
    }
}

impl LiftEx for Closure {
    fn lift(self, levels: u32) -> Self {
        use super::Closure::*;
        match self {
            Plain(body) => Self::plain(body.lift(levels)),
            Tree(split) => Tree(lift_tree_map(levels, split)),
        }
    }

    fn calc_level(&self) -> LevelCalcState {
        use super::Closure::*;
        match self {
            Plain(body) => body.calc_level(),
            Tree(split) => calc_tree_map_level(&split),
        }
    }
}
