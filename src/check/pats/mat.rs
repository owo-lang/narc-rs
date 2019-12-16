use std::{collections::HashMap, hash::BuildHasher, iter::once, ops::Add, rc::Rc};

use voile_util::uid::DBI;

use crate::{
    check::{
        monad::TCS,
        pats::{Blocked, CoreCopat, CorePat, Simpl},
        rules::ERROR_MSG,
    },
    syntax::{
        core::{subst::Subst, Elim, Term},
        pat::{Copat, Pat},
    },
};

/// If matching is inconclusive ([`Dunno`](self::Match::Dunno)) we want to know whether
/// it is due to a particular meta variable.
#[derive(Debug, Clone)]
pub enum Match {
    Yes(Simpl, HashMap<DBI, Term>),
    /// Don't know.
    Dunno(Blocked<()>),
    No,
}

impl Default for Match {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl Add for Match {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Match::Dunno(a), Match::Dunno(b)) => Match::Dunno(a + b),
            (_, Match::Dunno(b)) | (Match::Dunno(b), _) => Match::Dunno(b),
            (o, Match::No) | (Match::No, o) => o,
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
pub fn build_subst<H: BuildHasher>(map: HashMap<DBI, Term, H>, max: usize) -> Rc<Subst> {
    Subst::parallel(matched(map, max).into_iter())
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Patterns.Match.html#matchedArgs).
fn matched<T, H: BuildHasher>(mut map: HashMap<DBI, T, H>, max: usize) -> Vec<T> {
    (0..max)
        .map(DBI)
        .map(|i| map.remove(&i).expect(ERROR_MSG))
        .collect()
}

pub fn match_copats(
    tcs: &TCS,
    mut p: impl ExactSizeIterator<Item = (CoreCopat, Elim)>,
) -> (Match, Vec<Elim>) {
    let mut mat = Match::with_capacity(p.len());
    let mut elims = Vec::with_capacity(p.len());
    while let Some((copat, elim)) = p.next() {
        let (m, e) = match_copat(tcs, copat, elim);
        match m {
            Match::No if e.is_proj() => {
                elims.push(e);
                mat = Match::No;
                break;
            }
            Match::No => {
                // Agda#2964: Even when the first pattern doesn't match we should
                // continue to the next patterns (and potentially block on them)
                // because the splitting order in the case tree may not be
                // left-to-right.
                let copy = p.collect::<Vec<_>>();
                let mut copied_elims = copy.iter().map(|(_, e)| e).cloned().collect();
                let (m, _) = match_copats(tcs, copy.into_iter());
                mat = m;
                elims.append(&mut copied_elims);
                break;
            }
            Match::Dunno(d) => {
                mat = Match::Dunno(d);
                elims.extend(p.map(|(_, e)| e));
                break;
            }
            Match::Yes(a, b) => {
                mat = mat + Match::Yes(a, b);
                elims.push(e);
            }
        }
    }
    (mat, elims)
}

fn match_copat(tcs: &TCS, p: CoreCopat, e: Elim) -> (Match, Elim) {
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
        (Copat::App(p), Elim::App(t)) => {
            let (m, t) = match_pat(tcs, p, *t);
            (m, Elim::app(t))
        }
    }
}

fn match_pat(tcs: &TCS, p: CorePat, t: Term) -> (Match, Term) {
    match (p, t) {
        (Pat::Var(i), t) => (Match::Yes(Simpl::No, once((i, t.clone())).collect()), t),
        (Pat::Forced(_), t) => (Match::Yes(Simpl::No, Default::default()), t),
        (Pat::Absurd, _) => unreachable!(),
        _ => unimplemented!(),
    }
}
