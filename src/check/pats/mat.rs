use std::collections::HashMap;
use std::hash::BuildHasher;
use std::ops::Add;
use std::rc::Rc;

use voile_util::uid::DBI;

use crate::check::pats::CoreCopat;
use crate::check::rules::ERROR_MSG;
use crate::syntax::core::subst::Subst;
use crate::syntax::core::{Elim, Term};
use crate::syntax::pat::Copat;

/// `Simplification` in
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Simplification).
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Simpl {
    Yes,
    No,
}

impl From<bool> for Simpl {
    fn from(b: bool) -> Self {
        if b {
            Simpl::Yes
        } else {
            Simpl::No
        }
    }
}

impl Into<bool> for Simpl {
    fn into(self) -> bool {
        match self {
            Simpl::Yes => true,
            Simpl::No => false,
        }
    }
}

impl Default for Simpl {
    fn default() -> Self {
        Simpl::No
    }
}

impl Add for Simpl {
    type Output = Simpl;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Simpl::Yes => Simpl::Yes,
            Simpl::No => rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Match {
    Yes(Simpl, HashMap<DBI, Term>),
    No,
}

impl Default for Match {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl Add for Match {
    type Output = Match;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Match::No, o) => o,
            (o, Match::No) => o,
            (Match::Yes(s0, mut m0), Match::Yes(s1, m1)) => {
                m0.extend(m1.into_iter());
                Match::Yes(s0 + s1, m0)
            }
        }
    }
}

impl Match {
    pub fn with_capacity(capacity: usize) -> Self {
        Match::Yes(Default::default(), HashMap::with_capacity(capacity))
    }
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Patterns.Match.html#buildSubstitution).
pub fn build_subst<H: BuildHasher>(map: HashMap<DBI, Term, H>, max: DBI) -> Rc<Subst> {
    Subst::parallel(matched(map, max).into_iter())
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Patterns.Match.html#matchedArgs).
fn matched<T, H: BuildHasher>(mut map: HashMap<DBI, T, H>, DBI(max): DBI) -> Vec<T> {
    (0..max)
        .map(DBI)
        .map(|i| map.remove(&i).expect(ERROR_MSG))
        .collect()
}

fn match_copats(mut p: impl ExactSizeIterator<Item = (CoreCopat, Elim)>) -> (Match, Vec<Elim>) {
    let mut mat = Match::with_capacity(p.len());
    let mut elims = Vec::with_capacity(p.len());
    while let Some((copat, elim)) = p.next() {
        let (m, e) = match_copat(copat, elim);
        match m {
            Match::No if e.is_proj() => {
                elims.push(e);
                mat = Match::No;
                break;
            }
            Match::No => {
                let copy = p.collect::<Vec<_>>();
                let (m, _) = match_copats(copy.clone().into_iter());
                mat = m;
                elims.append(&mut copy.into_iter().map(|(_, e)| e).collect());
                break;
            }
            yes => {
                mat = mat + yes;
                elims.push(e);
            }
        }
    }
    (mat, elims)
}

fn match_copat(p: CoreCopat, e: Elim) -> (Match, Elim) {
    match (p, e) {
        (Copat::Proj(s0), Elim::Proj(s1)) => {
            if s0 == s1 {
                (Match::Yes(Simpl::Yes, Default::default()), Elim::Proj(s1))
            } else {
                (Match::No, Elim::Proj(s1))
            }
        }
        (Copat::Proj(..), Elim::App(a)) => (Match::No, Elim::App(a)),
        (Copat::App(..), Elim::Proj(s)) => (Match::No, Elim::Proj(s)),
        (Copat::App(a0), Elim::App(a1)) => unimplemented!(),
    }
}
